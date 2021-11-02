use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::{Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};

use layout_evaluation::evaluation::Evaluator;
use keyboard_layout::layout_generator::NeoLayoutGenerator;

use rocket::response::status::BadRequest;

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::SqlitePool);

// type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;
type Result<T, E = BadRequest<String>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
struct LayoutEvaluation {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    layout: String,
    total_cost: f64,
    details_json: String,
}

#[post("/", data = "<layout>")]
async fn get_or_create(
    mut db: Connection<Db>,
    layout: &str,
    layout_generator: &State<NeoLayoutGenerator>,
    evaluator: &State<Evaluator>,
) -> Result<Created<Json<LayoutEvaluation>>> {
    let result =
        sqlx::query_as::<_, LayoutEvaluation>("SELECT * FROM layouts WHERE layout = ?")
            .bind(layout)
            .fetch_one(&mut *db)
            .await
            .ok();

    let result = match result {
        None => {
            // TODO: proper error handling
            let l = layout_generator.generate(layout)
                .map_err(|e| BadRequest(Some(e.to_string())))?;
            let evaluation_result = evaluator.evaluate_layout(&l);

            let result = LayoutEvaluation {
                id: None,
                layout: layout.to_string(),
                total_cost: evaluation_result.total_cost(),
                // TODO: proper error handling
                details_json: serde_json::to_string(&evaluation_result)
                    .map_err(|e| BadRequest(Some(e.to_string())))?,
            };

            sqlx::query("INSERT INTO layouts (layout, total_cost, details_json) VALUES (?, ?, ?)")
                .bind(&result.layout)
                .bind(&result.total_cost)
                .bind(&result.details_json)
                .execute(&mut *db)
                .await
                .map_err(|e| BadRequest(Some(e.to_string())))?;

            result
        }
        Some(result) => result,
    };

    Ok(Created::new("/").body(Json(result)))
}

#[get("/")]
async fn list(mut db: Connection<Db>) -> Result<Json<Vec<LayoutEvaluation>>> {
    let layouts = sqlx::query_as::<_, LayoutEvaluation>("SELECT * FROM layouts")
        .fetch_all(&mut *db)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(layouts))
}

// #[get("/<id>")]
// async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<LayoutEvaluation>> {
//     sqlx::query!("SELECT id, layout, total_cost, details_json FROM layouts WHERE id = ?", id)
//         .fetch_one(&mut *db)
//         .map_ok(|r| Json(LayoutEvaluation { id: Some(r.id), layout: r.layout, total_cost: r.total_cost, details_json: r.details_json }))
//         .await
//         .ok()
// }

#[delete("/<layout>")]
async fn delete(mut db: Connection<Db>, layout: &str) -> Result<Option<()>> {
    let result = sqlx::query("DELETE FROM layouts WHERE layout = ?")
        .bind(layout)
        .execute(&mut *db)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok((result.rows_affected() == 1).then(|| ()))
}

// #[delete("/")]
// async fn destroy(mut db: Connection<Db>) -> Result<()> {
//     sqlx::query!("DELETE FROM layouts").execute(&mut *db).await?;

//     Ok(())
// }

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
            .mount("/", routes![list, get_or_create, delete])
    })
}
