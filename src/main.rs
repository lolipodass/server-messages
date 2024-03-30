mod entities;
mod setup;

use entities::{prelude::*, *};
use rocket::{
    fs::{relative, FileServer},
    *,
};
use rocket_dyn_templates::Template;
use sea_orm::*;
use sea_orm_migration::MigratorTrait;
use serde_json::json;
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

    rocket::build()
        .manage(db)
        .mount("/", FileServer::from(relative!("/static")))
        .mount(
            "/",
            routes![index, bakeries, bakery_by_id, new, new_bakery, reset],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
