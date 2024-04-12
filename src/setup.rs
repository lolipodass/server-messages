use sea_orm::*;

// Replace with your database URL and database name
<<<<<<< HEAD
const DATABASE_URL: &str = "postgres://postgres:1122@localhost:5505";
const DB_NAME: &str = "postgres";
=======
<<<<<<< HEAD
const DATABASE_URL: &str = "postgres://postgres:1122@localhost:5505";
const DB_NAME: &str = "postgres";
=======
const DATABASE_URL: &str = "postgres://postgres:123123@localhost:5432"; 
const DB_NAME: &str = "postr";
>>>>>>> 1ff8931 (add)
>>>>>>> f52da21 (Add)

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            // db.execute(Statement::from_string(
            //     db.get_database_backend(),
            //     format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            // ))
            // .await?;
            // db.execute(Statement::from_string(
            //     db.get_database_backend(),
            //     format!("CREATE DATABASE \"{}\";", DB_NAME),
            // ))
            // .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}
