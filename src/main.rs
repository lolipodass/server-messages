#[macro_use]
extern crate rocket;
use entities::user::Model;
use rocket::http::Status;
use rocket::response::status;

use rocket::serde::json::Json;
use rocket::State;

mod entities;
mod setup;

use entities::prelude::*;

use sea_orm::*;
use setup::set_up_db;

#[post("/addUser", data = "<user_data>", format = "json")]
async fn add_user(
    db: &State<DatabaseConnection>,
    user_data: Json<Model>,
) -> Result<Json<String>, Status> {
    let db = db as &DatabaseConnection;
    let user = entities::user::ActiveModel {
        name: ActiveValue::set(user_data.name.clone()),
        email: ActiveValue::set(user_data.email.clone()),
        password: ActiveValue::set(user_data.password.clone()),
        ..Default::default()
    }
    .insert(db)
    .await;

    match user {
        Ok(_) => Ok(Json("User added successfully".to_string())),
        Err(e) => {
            eprintln!("Failed to add user: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

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
        .mount("/", routes![test, check_connection, add_user])
}
