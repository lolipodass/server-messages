#[macro_use]
extern crate rocket;
use entities::user::Model;
use rocket::http::Status;
use rocket::response::status;
<<<<<<< HEAD
=======
<<<<<<< HEAD
=======
use entities::message::{self, Model as Message};
>>>>>>> 1ff8931 (add)
>>>>>>> f52da21 (Add)

use rocket::serde::json::Json;
use rocket::State;

mod entities;
mod setup;

use entities::prelude::*;

use sea_orm::*;
use setup::set_up_db;

<<<<<<< HEAD
=======
<<<<<<< HEAD
=======

#[post("/addMessage", data = "<message_data>", format = "json")]
pub async fn add_message(
    db: &State<DatabaseConnection>,
    message_data: Json<entities::message::Model>,
) -> Result<Json<String>, Status> {
    let db = db as &DatabaseConnection;

    // Проверяем существование отправителя сообщения
    let sender = User::find_by_id(message_data.sender_id)
        .one(db)
        .await;

    match sender {
        Ok(Some(_)) => {
            // Если отправитель существует, продолжаем создание сообщения
            let message = entities::message::ActiveModel {
                sender_id: ActiveValue::set(message_data.sender_id),
                receiver_id: ActiveValue::set(message_data.receiver_id),
                message: ActiveValue::set(message_data.message.clone()),
                ..Default::default()
            }
            .insert(db)
            .await;

            match message {
                Ok(_) => Ok(Json("Message added successfully".to_string())),
                Err(e) => {
                    eprintln!("Failed to add message: {}", e);
                    Err(Status::InternalServerError)
                }
            }
        },
        Ok(None) => {
            // Если отправитель не найден, возвращаем ошибку
            eprintln!("Sender not found");
            Err(Status::BadRequest)
        },
        Err(e) => {
            eprintln!("Failed to find sender: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/getMessagesFromSender/<sender_id>")]
pub async fn get_messages_from_sender(
    db: &State<DatabaseConnection>,
    sender_id: i32,
) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let messages = entities::message::Entity::find()
        .filter(entities::message::Column::SenderId.eq(sender_id))
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.message)
        .collect::<Vec<String>>();

    Json(messages)
}



>>>>>>> 1ff8931 (add)
>>>>>>> f52da21 (Add)
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

<<<<<<< HEAD
=======
<<<<<<< HEAD
=======

>>>>>>> 1ff8931 (add)
>>>>>>> f52da21 (Add)
#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
<<<<<<< HEAD
        .mount("/", routes![test, check_connection, add_user])
=======
<<<<<<< HEAD
        .mount("/", routes![test, check_connection, add_user])
=======
        .mount("/", routes![test, check_connection, add_user,add_message,get_messages_from_sender])
>>>>>>> 1ff8931 (add)
>>>>>>> f52da21 (Add)
}
