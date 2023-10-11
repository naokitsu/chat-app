mod auth;

#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket::http::Status;
use rocket::serde::json::serde_json::{json, Value};
use rocket::serde::{json, Deserialize, Serialize};
use sqlx::SqlitePool;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("Couldn't get the database URL from the environment");

    let pool = SqlitePool::connect(database_url.as_str())
        .await
        .expect("Couldn't connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    rocket::build()
        .mount("/", routes![auth::register_user])
        .manage(pool)
}
