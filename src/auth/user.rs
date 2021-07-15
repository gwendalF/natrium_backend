use crate::errors::{AppError, Result};
use argon2::ThreadMode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
#[derive(Debug, Serialize)]
pub struct User {
    pub uid: i64,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct PostUser {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(pool: &PgPool, email: &str, password: &str) -> Result<User> {
        let previous_user = sqlx::query!("SELECT email FROM users WHERE email=$1", email)
            .fetch_optional(pool)
            .await?;
        if previous_user.is_some() {
            return Err(AppError::AlreadyExist);
        } else {
            // hash password
            let config = argon2::Config {
                variant: argon2::Variant::Argon2id,
                lanes: 8,
                hash_length: 16,
                thread_mode: ThreadMode::Parallel,
                ..argon2::Config::default()
            };

            let mut salt = [0u8; 8];
            rand::thread_rng().fill(&mut salt);
            let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;
            println!("hash: {}", &hash);
            let uid = sqlx::query!(
                "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING uid",
                email,
                &hash,
            )
            .fetch_one(pool)
            .await?
            .uid;
            Ok(User {
                uid,
                email: email.to_owned(),
                password: Some(password.to_owned()),
            })
        }
    }
}
