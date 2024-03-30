#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status;
use sea_orm::Database;

use rocket::serde::json::Json;
use rocket::State;

mod entities;
mod setup;

use entities::prelude::*;

use sea_orm::*;
use setup::set_up_db;

#[get("/getAllUsers")]
async fn test(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let users = User::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();

    Json(users)
}
#[get("/ping")]
async fn check_connection(
    db: &State<DatabaseConnection>,
) -> Result<status::Custom<String>, status::Custom<String>> {
    let db = db as &DatabaseConnection;
    let users = User::find().all(db).await;

    match users {
        Ok(_) => {
            println!("Connection check successful");
            Ok(status::Custom(
                Status::Ok,
                "Connection check successful".to_string(),
            ))
        }
        Err(e) => {
            println!("Connection check failed: {}", e);
            Err(status::Custom(
                Status::InternalServerError,
                format!("Connection check failed: {}", e),
            ))
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![test, check_connection])
}
