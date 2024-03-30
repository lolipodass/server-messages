#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;

mod entities;
mod setup;

use entities::prelude::*;

use sea_orm::*;
use setup::set_up_db;

#[get("/test")]
async fn test(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let bakery_names = User::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();

    Json(bakery_names)
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build().manage(db).mount("/", routes![test])
}
