mod auth;

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::serde_json::{json, Value};
use rocket::serde::{json, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![auth::register_user])
}
