use crate::AppError;

use actix_web::{dev::ServiceRequest, web};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, EncodingKey, Validation};

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

use super::errors::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderClaims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub email: String,
    pub email_verified: bool,
}

pub enum TokenType {
    RefreshToken,
    AccessToken,
}

impl Claims {
    pub fn new(id: i32, token_type: TokenType) -> Self {
        let exp;
        let permissions;
        match token_type {
            TokenType::AccessToken => {
                exp = usize::try_from((Utc::now() + Duration::minutes(10)).timestamp()).unwrap();
                permissions = Some(vec![format!("READ_{}", id)]);
            }

            TokenType::RefreshToken => {
                // exp = usize::try_from((Utc::now() + Duration::days(7)).timestamp()).unwrap();
                exp = usize::try_from((Utc::now() + Duration::hours(1)).timestamp()).unwrap();
                println!("Wrong duration for refresh token");
                permissions = Some(vec![format!("ACCESS_TOKEN_{}", id)]);
            }
        }
        Claims {
            aud: "natrium".to_owned(),
            sub: id.to_string(),
            exp,
            iss: "natrium".to_owned(),
            permissions,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppKey {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey<'static>,
}

#[derive(Debug, Clone)]
pub struct RefreshKey {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey<'static>,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    if let Some(app_key) = req.app_data::<web::Data<AppKey>>() {
        let mut validation = Validation {
            iss: Some("natrium".to_owned()),
            ..Validation::default()
        };
        validation.set_audience(&["natrium"]);
        match decode::<Claims>(credentials.token(), &app_key.decoding, &validation) {
            Ok(token) => {
                if let Some(perm) = token.claims.permissions {
                    req.attach(perm);
                }
                Ok(req)
            }
            Err(e) => match e.kind() {
                ErrorKind::ExpiredSignature => Err(AppError::from(AuthError::ExpiredToken).into()),
                _ => Err(AppError::from(AuthError::Token).into()),
            },
        }
    } else {
        Err(AppError::ServerError.into())
    }
}
