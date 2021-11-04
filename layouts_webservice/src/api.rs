use super::Options;

use rocket::fairing::{self, AdHoc};
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::{Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database, Pool};

use keyboard_layout::layout_generator::NeoLayoutGenerator;
use layout_evaluation::evaluation::Evaluator;
use layout_evaluation::results::EvaluationResult;

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
    details_json: Option<String>,
    published_by: Option<String>,
    highlight: bool,
}

#[derive(Debug, Clone, Serialize)]
struct LayoutEvaluation {
    layout: String,
    total_cost: f64,
    published_by: Option<String>,
    details: Option<EvaluationResult>,
    plot: Option<String>,
    highlight: bool,
}

impl From<LayoutEvaluationDB> for LayoutEvaluation {
    fn from(item: LayoutEvaluationDB) -> Self {
        Self {
            layout: item.layout,
            total_cost: item.total_cost,
            published_by: item.published_by,
            details: item.details_json.map(|d| serde_json::from_str(&d).unwrap()),
            plot: None,
            highlight: item.highlight,
        }
    }
}

#[derive(Debug, Deserialize)]
struct PostLayout {
    layout: String,
    published_by: Option<String>,
}

#[post("/", data = "<layout>")]
async fn post(
    mut db: Connection<Db>,
    layout: Json<PostLayout>,
    layout_generator: &State<NeoLayoutGenerator>,
    evaluator: &State<Evaluator>,
) -> Result<Created<Json<LayoutEvaluation>>> {
    let l = layout_generator
        .generate(&layout.layout)
        .map_err(|_| Status::BadRequest)?;
    let layout_str = l.as_text();

    let result = sqlx::query_as::<_, LayoutEvaluationDB>("SELECT * FROM layouts WHERE layout = $1")
        .bind(&layout_str)
        .fetch_one(&mut *db)
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
                details_json: Some(
                    serde_json::to_string(&evaluation_result)
                        .map_err(|_| Status::InternalServerError)?,
                ),
                highlight: false,
            };

            sqlx::query("INSERT INTO layouts (layout, total_cost, published_by, details_json, highlight, created) VALUES ($1, $2, $3, $4, $5, NOW())")
                .bind(&result.layout)
                .bind(&result.total_cost)
                .bind(&result.published_by)
                .bind(&result.details_json)
                .bind(&result.highlight)
                .execute(&mut *db)
                .await
                .map_err(|_| Status::InternalServerError)?;

            result
        }
        Some(result) => result,
    };

    Ok(Created::new("/").body(Json(result.into())))
}

#[get("/")]
async fn list(mut db: Connection<Db>) -> Result<Json<Vec<LayoutEvaluation>>> {
    let layouts = sqlx::query_as::<_, LayoutEvaluationDB>(
        "SELECT NULL AS id, layout, total_cost, published_by, NULL AS details_json, highlight FROM layouts",
    )
    .fetch_all(&mut *db)
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

#[get("/<layout>")]
async fn get(
    mut db: Connection<Db>,
    layout: &str,
    layout_generator: &State<NeoLayoutGenerator>,
) -> Option<Json<LayoutEvaluation>> {
    sqlx::query_as::<_, LayoutEvaluationDB>(
        "SELECT NULL AS id, layout, total_cost, published_by, details_json, highlight FROM layouts WHERE layout = $1",
    )
    .bind(layout)
    .fetch_one(&mut *db)
    .await
    .map(|e| {
        let mut e: LayoutEvaluation = e.into();
        let l = layout_generator.generate(&e.layout).unwrap();
        e.plot = Some(l.plot());
        Json(e)
    })
    .ok()
}

#[delete("/<layout>")]
async fn delete(mut db: Connection<Db>, layout: &str) -> Result<Option<()>> {
    let result = sqlx::query("DELETE FROM layouts WHERE layout = $1")
        .bind(layout)
        .execute(&mut *db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok((result.rows_affected() == 1).then(|| ()))
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

async fn reeval_layouts(rocket: Rocket<Build>) -> fairing::Result {
    match (
        &rocket.state::<NeoLayoutGenerator>(),
        &rocket.state::<Evaluator>(),
        &rocket.state::<Options>(),
        Db::fetch(&rocket),
    ) {
        (Some(layout_generator), Some(evaluator), Some(options), Some(db)) => {
            if options.reeval_layouts {
                println!("Reevaluating results");
                let mut connection = db.0.get().await.unwrap();
                let results: Vec<LayoutEvaluationDB> = sqlx::query_as::<_, LayoutEvaluationDB>(
                    "SELECT id, layout, total_cost, published_by, NULL AS details_json FROM layouts",
                )
                .fetch_all(&mut connection)
                .await
                .unwrap();

                for result in results {
                    let layout = layout_generator.generate(&result.layout).unwrap();
                    let evaluation_result = evaluator.evaluate_layout(&layout);
                    let total_cost = evaluation_result.total_cost();
                    let details_json = Some(serde_json::to_string(&evaluation_result).unwrap());

                    println!(
                        "Re-evaluated {} (id: {}) from {:>.2} to {:>.2}",
                        result.layout,
                        result.id.unwrap(),
                        result.total_cost,
                        total_cost
                    );
                    sqlx::query("UPDATE layouts SET total_cost = ?, details_json = ? WHERE id = ?")
                        .bind(&total_cost)
                        .bind(&details_json)
                        .bind(&result.id)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
            }
        }
        _ => {}
    };

    Ok(rocket)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .attach(AdHoc::try_on_ignite("Reeval Layouts", reeval_layouts))
            .mount("/", routes![list, post, get, delete])
    })
}
