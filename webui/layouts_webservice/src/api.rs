use super::Options;

use keyboard_layout::layout_generator::LayoutGenerator;
use keyboard_layout::neo_layout_generator::NeoLayoutGenerator;
use layout_evaluation::{evaluation::Evaluator, results::EvaluationResult};

use ahash::AHashMap;
use rocket::{
    fairing::{self, AdHoc},
    http::Status,
    response::status::Created,
    serde::{json::Json, Deserialize, Serialize},
    State, {Build, Rocket},
};
use rocket_db_pools::{sqlx, Connection, Database};

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::PgPool);

// type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;
type Result<T, E = Status> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
struct LayoutEvaluationDB {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    layout: String,
    total_cost: f64,
    details_json: String,
    printed: String,
    published_by: Option<String>,
    highlight: bool,
    layout_config: String,
}

#[derive(Debug, Clone, Serialize)]
struct LayoutEvaluation {
    layout: String,
    total_cost: f64,
    published_by: Option<String>,
    details: Option<EvaluationResult>,
    printed: Option<String>,
    plot: Option<String>,
    highlight: bool,
    layout_config: String,
}

impl From<LayoutEvaluationDB> for LayoutEvaluation {
    fn from(item: LayoutEvaluationDB) -> Self {
        Self {
            layout: item.layout,
            total_cost: item.total_cost,
            published_by: item.published_by,
            details: None,
            printed: None,
            plot: None,
            highlight: item.highlight,
            layout_config: item.layout_config,
        }
    }
}

#[derive(Debug, Deserialize)]
struct PostLayout {
    layout: String,
    published_by: Option<String>,
    highlight: Option<bool>,
    secret: Option<String>,
    layout_config: Option<String>,
}

#[options("/")]
fn cors_preflight() {}

#[post("/", data = "<layout>")]
async fn post(
    mut db: Connection<Db>,
    layout: Json<PostLayout>,
    layout_generators: &State<AHashMap<String, NeoLayoutGenerator>>,
    evaluator: &State<Evaluator>,
    config: &State<Options>,
) -> Result<Created<Json<LayoutEvaluation>>> {
    // check if highlight wants to be set without permission
    let is_admin = config.secret == layout.secret.clone().unwrap_or_default();
    let highlight = layout.highlight.unwrap_or(false);
    if highlight && !is_admin {
        return Err(Status::Forbidden);
    };

    // generate layout
    let layout_config = layout
        .layout_config
        .clone()
        .unwrap_or_else(|| config.default_layout_config.to_owned());
    let layout_generator = layout_generators
        .get(&layout_config)
        .ok_or(Status::BadRequest)?;

    let layout_str: String = layout
        .layout
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let l = layout_generator
        .generate(&layout_str)
        .map_err(|_| Status::BadRequest)?;

    // check if layout is in database already
    let result = sqlx::query_as::<_, LayoutEvaluationDB>(
        "SELECT * FROM layouts WHERE layout = $1 AND layout_config = $2",
    )
    .bind(&layout_str)
    .bind(&layout_config)
    .fetch_one(&mut **db)
    .await
    .ok();

    let result = match result {
        None => {
            println!("Evaluating new layout: {}", layout_str);
            let evaluation_result = evaluator.evaluate_layout(&l);

            let result = LayoutEvaluationDB {
                id: None,
                layout: layout_str,
                total_cost: evaluation_result.total_cost(),
                published_by: layout.published_by.clone(),
                details_json: serde_json::to_string(&evaluation_result)
                    .map_err(|_| Status::InternalServerError)?,
                printed: format!("{}", evaluation_result),
                highlight,
                layout_config,
            };

            sqlx::query("INSERT INTO layouts (layout, total_cost, published_by, details_json, printed, highlight, layout_config, created) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())")
                .bind(&result.layout)
                .bind(result.total_cost)
                .bind(&result.published_by)
                .bind(&result.details_json)
                .bind(&result.printed)
                .bind(result.highlight)
                .bind(&result.layout_config)
                .execute(&mut **db)
                .await
                .map_err(|_| Status::InternalServerError)?;

            result
        }
        Some(result) => result,
    };

    Ok(Created::new("/").body(Json(result.into())))
}

#[get("/?<layout_config>")]
async fn list(
    layout_config: Option<String>,
    config: &State<Options>,
    mut db: Connection<Db>,
) -> Result<Json<Vec<LayoutEvaluation>>> {
    let layout_config = layout_config.unwrap_or_else(|| config.default_layout_config.to_owned());
    let layouts = sqlx::query_as::<_, LayoutEvaluationDB>(
        "SELECT NULL AS id, layout, total_cost, published_by, details_json, printed, highlight, layout_config FROM layouts WHERE layout_config = $1",
    )
    .bind(&layout_config)
    .fetch_all(&mut **db)
    .await
    .map_err(|e| {
        eprintln!("Error while fetching all layouts from db: {:?}", e);
        Status::InternalServerError
    })?
    .into_iter()
    .map(|e| e.into())
    .collect();

    Ok(Json(layouts))
}

#[get("/<layout>?<layout_config>")]
async fn get(
    mut db: Connection<Db>,
    layout: &str,
    layout_config: Option<String>,
    layout_generators: &State<AHashMap<String, NeoLayoutGenerator>>,
    config: &State<Options>,
) -> Result<Json<LayoutEvaluation>> {
    let layout_config = layout_config.unwrap_or_else(|| config.default_layout_config.to_owned());
    let layout_generator = layout_generators
        .get(&layout_config)
        .ok_or(Status::BadRequest)?;

    sqlx::query_as::<_, LayoutEvaluationDB>(
        "SELECT NULL AS id, layout, total_cost, published_by, details_json, printed, highlight, layout_config FROM layouts WHERE layout = $1 AND layout_config = $2",
    )
    .bind(layout.replace("__", "/"))
    .bind(&layout_config)
    .fetch_one(&mut **db)
    .await
    .map_err(|e| {
        eprintln!("Error while fetching layout from db: {:?}", e);
        Status::InternalServerError
    })
    .map(|e| {
        let mut res: LayoutEvaluation = e.clone().into();
        let l = layout_generator
                .generate(&e.layout).unwrap();
        res.plot = Some(l.plot());
        res.details = Some(serde_json::from_str(&e.details_json).unwrap());
        res.printed = Some(e.printed);
        Json(res)
    })
}

#[post("/reeval", data = "<secret>")]
async fn reeval(
    mut db: Connection<Db>,
    secret: &str,
    layout_generators: &State<AHashMap<String, NeoLayoutGenerator>>,
    evaluator: &State<Evaluator>,
    config: &State<Options>,
) -> Result<()> {
    let is_admin = config.secret == *secret;
    if !is_admin {
        println!("Wrong password provided for re-evaluation.");
        return Err(Status::Unauthorized);
    }

    println!("Reevaluating results");
    let results: Vec<LayoutEvaluationDB> = sqlx::query_as::<_, LayoutEvaluationDB>(
        "SELECT id, layout, total_cost, details_json, printed, published_by, highlight, layout_config FROM layouts",
    )
    .fetch_all(&mut **db)
    .await
    .map_err(|_| Status::InternalServerError)?;

    for result in results {
        let layout_generator = layout_generators
            .get(&result.layout_config)
            .ok_or(Status::BadRequest)?;
        let layout = layout_generator.generate(&result.layout).unwrap();
        let evaluation_result = evaluator.evaluate_layout(&layout);
        let total_cost = evaluation_result.total_cost();
        let details_json = Some(serde_json::to_string(&evaluation_result).unwrap());
        let printed = format!("{}", evaluation_result);

        println!(
            "Re-evaluated {} (id: {}) from {:>.2} to {:>.2}",
            result.layout,
            result.id.unwrap(),
            result.total_cost,
            total_cost
        );
        sqlx::query(
            "UPDATE layouts SET total_cost = $1, details_json = $2 , printed = $3 WHERE id = $4",
        )
        .bind(total_cost)
        .bind(&details_json)
        .bind(&printed)
        .bind(result.id)
        .execute(&mut **db)
        .await
        .map_err(|_| Status::InternalServerError)?;
    }

    Ok(())
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/api", routes![list, post, get, reeval, cors_preflight])
    })
}
