use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::NaiveDateTime;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

pub async fn validator<'a>(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> std::result::Result<ServiceRequest, actix_web::error::Error> {
    let google_key = req.app_data::<GoogleKey>();
    match google_key {
        Some(key) => match decode_jwt(credentials.token(), key) {
            Ok(_) => Ok(req),
            Err(e) => Err(e),
        },
        None => Err(AppError::DataError(
            "Missing public key to check authentication".to_owned(),
        ))?,
    }
}

fn decode_jwt(credentials: &str, key: &GoogleKey) -> Result<(), actix_web::Error> {
    let decoding_key = DecodingKey::from_rsa_components("psh4_fDTsNZ1JkC2BV6nsU7681neTu8D37bMwTzzT-hugnePDyLaR8a_2HnqJaABndr0793WQCkiDolIjX1wn0a6zTpdgCJL-vaFe2FqPg19TWsZ8O6oKZc_rtWu-mE8Po7RGzi9qPLv9FxJPbiGq_HnMUo0EG7J4sN3IuzbU--Wmuz8LWALwmfpE9CfOym8x5GdUzbDL1ltuC2zXCaxARDnPs6vKR6eW1MZgXqgQ6ZQO9FklH_b5WJYLBDmHAb6CguoeU-AozaoVrBHgkWoDkku7nMWoetULtgBP_tYtFM8zvJ9IDD6abZM0jl-bsHIm3XFz0MgAJ9FmPti9-iShQ", "AQAB");
    let token = decode::<Claims>(
        credentials,
        &decoding_key,
        &Validation::new(jsonwebtoken::Algorithm::RS256),
    )
    .expect("Cannot decode token");
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String,
}

#[derive(Clone)]
pub struct GoogleKey<'a> {
    pub key: DecodingKey<'a>,
    pub expiration: NaiveDateTime,
}
