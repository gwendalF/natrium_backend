use crate::AppError;

use actix_web::{
    dev::{Path, ServiceRequest},
    http::header::CacheDirective,
    web, FromRequest,
};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub permissions: Option<Vec<String>>,
}

impl Claims {
    pub fn new(id: i32) -> Self {
        let exp = usize::try_from((Utc::now() + Duration::hours(1)).timestamp()).unwrap();
        let permissions = Some(vec![format!("READ_{}", id)]);
        Claims {
            aud: "natrium".to_owned(),
            sub: id.to_string(),
            exp,
            iss: "natrium".to_owned(),
            permissions,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct AppKey {
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
                if let Some(perm) = token.claims.permissions {
                    req.attach(perm);
                }
                Ok(req)
            }
            Err(e) => Err(AppError::PermissionDenied(e.to_string()))?,
        }
    } else {
        Err(AppError::ServerError)?
    }
}