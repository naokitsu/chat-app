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

#[post("/login", data = "<user>")]
pub async fn login_user(
    pool: &State<Pool<Sqlite>>,
    user: json::Json<UserRegistration>,
) -> Result<json::Json<Token>, Status> {
    let mut connection = pool
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let query_result = sqlx::query!("SELECT * FROM users WHERE username = ?", user.username)
        .fetch_one(&mut *connection)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let real_hash = query_result.salted_hash;

    let salt = real_hash[..16].to_vec();

    let user_hash =
        hash_password(&*user.password, Some(salt)).map_err(|_| Status::InternalServerError)?;

    if user_hash == real_hash {
        let token = Token::generate_token().map_err(|_| Status::InternalServerError)?;
        Ok(json::Json(token))
    } else {
        Err(Status::Unauthorized)
    }
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
        hash_password(user.password.as_str(), None).map_err(|_| Status::InternalServerError)?;

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

    let token = Token::generate_token().map_err(|_| Status::InternalServerError)?;

    Ok(json::Json(token))
}

fn hash_password(
    password: &str,
    optional_salt: Option<Vec<u8>>,
) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let mut salt = [0u8; 16];
    if let Some(x) = optional_salt {
        salt.copy_from_slice(&x[..16])
    } else {
        rand_bytes(&mut salt).unwrap();
    }

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
