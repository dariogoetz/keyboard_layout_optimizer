use rocket::fairing::{self, AdHoc};
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::{Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};

use keyboard_layout::layout_generator::NeoLayoutGenerator;
use layout_evaluation::evaluation::Evaluator;
use layout_evaluation::results::EvaluationResult;

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::SqlitePool);

// type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;
type Result<T, E = Status> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
struct LayoutEvaluationDB {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    layout: String,
    total_cost: f64,
    details_json: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct LayoutEvaluation {
    layout: String,
    total_cost: f64,
    details: Option<EvaluationResult>,
}

impl From<LayoutEvaluationDB> for LayoutEvaluation {
    fn from(item: LayoutEvaluationDB) -> Self {
        Self {
            layout: item.layout,
            total_cost: item.total_cost,
            details: item.details_json.map(|d| serde_json::from_str(&d).unwrap()),
        }
    }
}

#[post("/", data = "<layout>")]
async fn post(
    mut db: Connection<Db>,
    layout: &str,
    layout_generator: &State<NeoLayoutGenerator>,
    evaluator: &State<Evaluator>,
) -> Result<Created<Json<LayoutEvaluation>>> {
    let result = sqlx::query_as::<_, LayoutEvaluationDB>("SELECT * FROM layouts WHERE layout = ?")
        .bind(layout)
        .fetch_one(&mut *db)
        .await
        .ok();

    let result = match result {
        None => {
            log::info!("Evaluating new layout: {}", layout);
            let l = layout_generator
                .generate(layout)
                .map_err(|_| Status::BadRequest)?;
            let evaluation_result = evaluator.evaluate_layout(&l);

            let result = LayoutEvaluationDB {
                id: None,
                layout: layout.to_string(),
                total_cost: evaluation_result.total_cost(),
                details_json: Some(
                    serde_json::to_string(&evaluation_result)
                        .map_err(|_| Status::InternalServerError)?,
                ),
            };

            sqlx::query("INSERT INTO layouts (layout, total_cost, details_json) VALUES (?, ?, ?)")
                .bind(&result.layout)
                .bind(&result.total_cost)
                .bind(&result.details_json)
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
        "SELECT NULL AS id, layout, total_cost, NULL AS details_json FROM layouts",
    )
    .fetch_all(&mut *db)
    .await
    .map_err(|_| Status::InternalServerError)?
    .into_iter()
    .map(|e| e.into())
    .collect();

    Ok(Json(layouts))
}

#[get("/<layout>")]
async fn get(mut db: Connection<Db>, layout: &str) -> Option<Json<LayoutEvaluation>> {
    sqlx::query_as::<_, LayoutEvaluationDB>(
        "SELECT NULL AS id, layout, total_cost, details_json FROM layouts WHERE layout = ?",
    )
    .bind(layout)
    .fetch_one(&mut *db)
    .await
    .map(|e| Json(e.into()))
    .ok()
}

#[delete("/<layout>")]
async fn delete(mut db: Connection<Db>, layout: &str) -> Result<Option<()>> {
    let result = sqlx::query("DELETE FROM layouts WHERE layout = ?")
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

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            // .mount("/sqlx", routes![list, create, read, delete, destroy])
            .mount("/", routes![list, post, get, delete])
    })
}
