use openssl::hash::{Hasher, MessageDigest};
use openssl::rand::rand_bytes;
use rocket::http::Status;
use rocket::serde::json::serde_json::{self, json, Value};
use rocket::serde::{json, Deserialize, Serialize};
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::io::ErrorKind::AlreadyExists;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserRegistration {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    token: String,
}

#[post("/register", data = "<user>")]
pub async fn register_user(
    pool: &State<Pool<Sqlite>>,
    user: json::Json<UserRegistration>,
) -> Result<json::Json<Token>, Status> {
    let mut connection = pool
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let query_result = sqlx::query!(
        "SELECT COUNT(*) as count FROM users WHERE username = ?",
        user.username
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|_| Status::InternalServerError)?;

    if query_result.count > 0 {
        return Err(Status::BadRequest);
    }

    let hashed_password =
        hash_password(user.password.as_str()).map_err(|_| Status::InternalServerError)?;

    let token = Token::generate_token().map_err(|_| Status::InternalServerError)?;

    let insert_result = sqlx::query!(
        "INSERT INTO users (username, salted_hash) VALUES (?, ?)",
        user.username,
        hashed_password
    )
    .execute(&mut *connection)
    .await
    .map_err(|e| {
        eprintln!("Error inserting user into the database: {:?}", e);
        Status::InternalServerError
    })?;

    Ok(json::Json(token))
}

fn hash_password(password: &str) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let mut salt = [0u8; 16];
    rand_bytes(&mut salt).unwrap();

    let mut hasher = Hasher::new(MessageDigest::sha256())?;
    hasher.update(password.as_bytes())?;
    hasher.update(&salt)?;
    let hash = hasher.finish()?;

    let mut result = Box::new([0u8; 48]);
    result[..16].copy_from_slice(&salt[..]);
    result[16..].copy_from_slice(&hash[..]);

    Ok(result.to_vec())
}

impl Token {
    fn generate_token() -> Result<Token, openssl::error::ErrorStack> {
        let mut random_bytes = [0u8; 32];
        rand_bytes(&mut random_bytes)?;
        let token = random_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        Ok(Token { token })
    }
}
