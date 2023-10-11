use openssl::hash::{Hasher, MessageDigest};
use openssl::rand::rand_bytes;
use rocket::http::Status;
use rocket::serde::json::serde_json::{json, Value};
use rocket::serde::{json, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistration {
    username: String,
    password: String,
}

#[post("/users", data = "<user>")]
pub async fn register_user(user: json::Json<UserRegistration>) -> Status {
    let hashed_password = match hash_password(user.password.as_str()) {
        Ok(x) => x,
        _ => return Status::InternalServerError,
    };

    println!(
        "New user registered: {:?}",
        (&user.username, &hashed_password)
    );

    Status::Created
}

fn hash_password(password: &str) -> Result<Box<[u8; 48]>, openssl::error::ErrorStack> {
    let mut salt = [0u8; 16];
    rand_bytes(&mut salt).unwrap();

    let mut hasher = Hasher::new(MessageDigest::sha256())?;
    hasher.update(password.as_bytes())?;
    hasher.update(&salt)?;
    let hash = hasher.finish()?;

    let mut result = Box::new([0u8; 48]);
    result[..16].copy_from_slice(&salt[..]);
    result[16..].copy_from_slice(&hash[..]);

    Ok(result)
}
