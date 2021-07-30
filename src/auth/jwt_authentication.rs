use std::{collections::HashMap, sync::Mutex};

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> std::result::Result<ServiceRequest, actix_web::error::Error> {
    let google_key_mutex = req
        .app_data::<actix_web::web::Data<Mutex<GoogleKeySet>>>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized(AppError::PermissionDenied))?;
    {
        let mut key = google_key_mutex
            .as_ref()
            .lock()
            .expect("Cannot acquire mutex public key");
        if key.expiration <= Utc::now().naive_utc() {
            // update public key
            let client = reqwest::Client::new();
            let response = client
                .get("https://www.googleapis.com/oauth2/v3/certs")
                .send()
                .await
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
            let expiration = response.headers()["cache-control"].to_str().unwrap();
            lazy_static! {
                static ref RE: regex::Regex = regex::Regex::new("(max-age=)([0-9]*)").unwrap();
            }
            let expiration_value = RE.captures(expiration).unwrap().get(2).unwrap().as_str();
            let expiration = (Utc::now()
                + Duration::seconds(expiration_value.parse::<i64>().unwrap()))
            .naive_utc();
            let key_set = response
                .json::<HashMap<String, Vec<HashMap<String, String>>>>()
                .await
                .unwrap();
            let db_pool = req
                .app_data::<actix_web::web::Data<PgPool>>()
                .unwrap()
                .as_ref();
            for (i, key) in key_set["keys"].iter().enumerate() {
                sqlx::query!(
                    "UPDATE token_key
                SET kid=$1,
                modulus=$2,
                exponent=$3,
                expiration=$4
                WHERE id=$5
                ",
                    key["kid"].clone(),
                    key["n"].clone(),
                    key["e"].clone(),
                    expiration,
                    (i + 1) as i32
                )
                .execute(db_pool)
                .await
                .unwrap();
                println!("Update db key");
            }
            *key = GoogleKeySet {
                expiration: (Utc::now() + Duration::days(3)).naive_local(),
                keys: key.keys.clone(),
            }
        }
    }
    // garder un key_set en mutex (permet d'accéder à n'importe quelle clé selon le token)
    Ok(req)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String,
}

#[derive(Clone, Debug)]
pub struct GoogleKeySet {
    pub keys: Vec<GoogleKey>,
    pub expiration: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct GoogleKey {
    pub kid: String,
    pub modulus: String,
    pub exponent: String,
}
