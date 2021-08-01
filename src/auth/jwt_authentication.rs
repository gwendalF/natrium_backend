use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    dev::{Path, ServiceRequest},
    http::header::CacheDirective,
    FromRequest,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
    iss: String,
}

#[derive(Clone, Debug)]
pub struct GoogleKeySet {
    pub keys: HashMap<String, DecodingKey<'static>>,
    pub expiration: NaiveDateTime,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> std::result::Result<ServiceRequest, actix_web::error::Error> {
    let google_key_mutex = req
        .app_data::<actix_web::web::Data<Mutex<GoogleKeySet>>>()
        .ok_or_else(|| AppError::PermissionDenied("application key unavailable".to_owned()))?;
    {
        let expiration;
        {
            let key = google_key_mutex
                .lock()
                .expect("Cannot acquire mutex public key");
            expiration = key.expiration;
        }
        let pool = req
            .app_data::<actix_web::web::Data<PgPool>>()
            .ok_or_else(|| AppError::DataError("DB pool not available".to_owned()))?
            .as_ref();
        if expiration <= Utc::now().naive_utc() {
            update_key(pool, google_key_mutex.as_ref()).await?;
        }
        let validation = Validation::new(Algorithm::RS256);
        let kid = decode_header(credentials.token())
            .map_err(|e| AppError::PermissionDenied(format!("{}", e)))?
            .kid
            .ok_or_else(|| AppError::PermissionDenied("JWT header has no kid key".to_owned()))?;
        let token = decode::<Claims>(
            credentials.token(),
            &google_key_mutex.lock().expect("Mutex blocked").keys[&kid],
            &validation,
        )
        .map_err(|e| AppError::PermissionDenied(e.to_string()))?;
        match token.claims.iss.as_str() {
            "accounts.google.com" => Ok(()),
            "https://accounts.google.com" => Ok(()),
            _ => Err(AppError::PermissionDenied("JWT issuer is wrong".to_owned())),
        }?;
        let user_id = req
            .match_info()
            .get("user_id")
            .ok_or_else(|| AppError::NotFoundError)?;
        let subject = sqlx::query!(
            "SELECT subject FROM provider_user_mapper
        JOIN user_account ON (provider_user_mapper.user_id=user_account.id)"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.into()))?
        .subject;
    }
    Ok(req)
}

async fn update_key(pool: &PgPool, key: &Mutex<GoogleKeySet>) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v3/certs")
        .send()
        .await?;
    let expiration = response.headers()["cache-control"].to_str()?;
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("(max-age=)([0-9]*)").unwrap();
    }
    let expiration_str = RE.captures(expiration).unwrap().get(2).unwrap().as_str();
    let expiration = (Utc::now() + Duration::seconds(expiration_str.parse().unwrap())).naive_utc();
    let key_set_response = response
        .json::<HashMap<String, Vec<HashMap<String, String>>>>()
        .await?;
    let mut keys = HashMap::with_capacity(key_set_response["keys"].len());
    for key in key_set_response["keys"].iter() {
        sqlx::query!(
            "INSERT INTO token_key (kid, modulus, exponent, expiration, provider)
        VALUES ($1,$2,$3,$4, 'google') 
        ON CONFLICT(kid) 
        DO UPDATE SET kid=$1, modulus=$2, exponent=$3, expiration=$4",
            key["kid"],
            key["n"],
            key["e"],
            expiration
        )
        .execute(pool)
        .await?;
        keys.insert(
            key["kid"].to_owned(),
            DecodingKey::from_rsa_components(&key["n"], &key["e"]).into_static(),
        );
    }
    let mut key = key.lock().expect("Cannot acquire mutext");
    *key = GoogleKeySet { keys, expiration };
    Ok(())
}
