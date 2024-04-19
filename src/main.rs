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
#[delete("/deleteUser/<id>")]
async fn delete_user(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<String>, Status> {
    let db = db as &DatabaseConnection;
    let user = entities::user::Entity::find_by_id(id)
        .one(db)
        .await;

    match user {
        Ok(Some(user)) => {
            user.delete(db).await.map_err(|e| {
                eprintln!("Failed to delete user: {}", e);
                Status::InternalServerError
            })?;
            Ok(Json("User deleted successfully".to_string()))
        }
        Ok(None) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Failed to find user: {}", e);
            Err(Status::InternalServerError)
        }
    }
}



#[post("/createMessage", data = "<message_data>", format = "json")] 
async fn create_message(
    db: &State<DatabaseConnection>,
    message_data: Json<entities::message::Model>, // Используйте модель message::Model
) -> Result<Json<String>, Status> {
    let db = db as &DatabaseConnection;

    // Проверяем существование отправителя и получателя
    let sender_exists = entities::user::Entity::find_by_id(message_data.sender_id)
        .one(db)
        .await
        .is_ok();

    let receiver_exists = entities::user::Entity::find_by_id(message_data.receiver_id)
        .one(db)
        .await
        .is_ok();

    if !sender_exists || !receiver_exists {
        return Err(Status::BadRequest);
    }

    // Если оба пользователя существуют, создаем сообщение
    let message = entities::message::ActiveModel {
        sender_id: ActiveValue::set(message_data.sender_id),
        receiver_id: ActiveValue::set(message_data.receiver_id),
        message: ActiveValue::set(message_data.message.clone()),
        ..Default::default() 
    }
    .insert(db)
    .await;

    match message {
        Ok(_) => Ok(Json("Message created successfully".to_string())),
        Err(e) => {
            eprintln!("Failed to create message: {}", e);
            Err(Status::InternalServerError)
        }
    }
}





use sea_orm::Condition;

#[get("/messages/<user_id>")] // Поиск по id получателя/отправителя
async fn get_messages_by_user_id(
    db: &State<DatabaseConnection>,
    user_id: i32,
) -> Result<Json<Vec<entities::message::Model>>, Status> {
    let db = db as &DatabaseConnection;
    let messages = entities::message::Entity::find()
        .filter(
            Condition::any()
                .add(entities::message::Column::SenderId.eq(user_id))
                .add(entities::message::Column::ReceiverId.eq(user_id))
        )
        .all(db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(messages))
}
#[get("/message/<message_id>")] // Поиск по id сообщения
async fn find_message_by_id(
    db: &State<DatabaseConnection>,
    message_id: i32,
) -> Result<Json<Option<entities::message::Model>>, Status> {
    let db = db as &DatabaseConnection;
    let message = entities::message::Entity::find_by_id(message_id)
        .one(db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    match message {
        Some(msg) => Ok(Json(Some(msg))),
        None => Ok(Json(None)),
    }
}

use sea_orm::DatabaseConnection;

#[delete("/deleteMessage/<message_id>")]
async fn delete_message(
    db: &State<DatabaseConnection>,
    message_id: i32,
) -> Result<Json<String>, Status> {
    let db = db as &DatabaseConnection;
    let result = entities::message::Entity::delete_by_id(message_id)
        .exec(db)
        .await;

    match result {
        Ok(_) => Ok(Json("Message deleted successfully".to_string())),
        Err(_) => Err(Status::InternalServerError),
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
        .mount("/", routes![test, check_connection, add_user,create_message,get_messages_by_user_id,delete_message, delete_user,find_message_by_id])
}
