use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::DatabaseConnection;

#[get("/test")]
async fn test(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let bakery_names = Bakery::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();

    Json(bakery_names)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        // Don't forget to mount the new endpoint handlers
        routes![
                    index,
        +           bakeries
                ],
    )
}
