use std::collections::HashMap;

use crate::domain::auth::value_object::provider_key::GoogleKeySet;
use crate::AppError;
use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::DecodingKey;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::CACHE_CONTROL;
use sqlx::PgPool;

use crate::domain::{self, auth::user::User};
use crate::Result;

pub async fn update_key_set(key_set: &mut GoogleKeySet, pool: &PgPool) -> Result<()> {
    if key_set.expiration >= Utc::now().naive_utc() {
        return Ok(());
    }
    let new_key_set = get_google_key_set().await?;
    for key_response in &new_key_set {
        upsert(pool, key_response).await?;
        if let Some(key) = key_set.keys.get_mut(&key_response.kid) {
            println!("Update decoding key state");
            *key = DecodingKey::from_rsa_components(&key_response.n, &key_response.e).into_static();
        }
    }
    Err(AppError::ServerError)
}

async fn upsert(pool: &PgPool, response: &KeySetResponse) -> Result<()> {
    sqlx::query!("");
    Ok(())
}

async fn get_google_key_set() -> Result<Vec<KeySetResponse>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("max-age=([0-9]+)").unwrap();
    }
    let response = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await?;
    let header = response.headers()[CACHE_CONTROL].clone();
    let capture = RE
        .captures(
            header
                .to_str()
                .expect("Google key header cache-control cannot be used as str"),
        )
        .ok_or_else(|| AppError::ServerError)?;
    let expiration = capture
        .get(1)
        .ok_or_else(|| AppError::ServerError)?
        .as_str();
    let expiration = NaiveDateTime::from_timestamp(expiration.parse::<i64>()?, 0);
    let new_key_set = response
        .json::<HashMap<String, Vec<HashMap<String, String>>>>()
        .await?;
    new_key_set["keys"]
        .iter()
        .map(|key_data| {
            Ok(KeySetResponse {
                n: key_data["n"].clone(),
                e: key_data["e"].clone(),
                kid: key_data["kid"].clone(),
                expiration,
            })
        })
        .collect()
}

struct KeySetResponse {
    n: String,
    e: String,
    kid: String,
    expiration: NaiveDateTime,
}
