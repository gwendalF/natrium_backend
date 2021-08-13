use std::{collections::HashMap, sync::Mutex};

use crate::errors::AppError;
use actix_web::{
    dev::{Path, ServiceRequest},
    http::header::CacheDirective,
    web, FromRequest,
};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, EncodingKey, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::convert::TryFrom;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub permissions: Vec<String>,
}

impl Claims {
    pub fn new(id: i32) -> Self {
        let exp = usize::try_from((Utc::now() + Duration::hours(1)).timestamp()).unwrap();
        let permissions = vec![format!("READ_{}", id)];
        Claims {
            aud: "natrium".to_owned(),
            sub: id.to_string(),
            exp,
            iss: "natrium".to_owned(),
            permissions,
        }
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct GoogleKeySet {
    pub keys: HashMap<String, DecodingKey<'static>>,
    pub expiration: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct JwtKey {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey<'static>,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    if let Some(decoding_key) = req.app_data::<web::Data<DecodingKey>>() {
        match decode::<Claims>(credentials.token(), decoding_key, &Validation::default()) {
            Ok(token) => {
                req.attach(token.claims.permissions);
                Ok(req)
            }
            Err(e) => Err(AppError::PermissionDenied(e.to_string()))?,
        }
    } else {
        Err(AppError::ServerError)?
    }
}
