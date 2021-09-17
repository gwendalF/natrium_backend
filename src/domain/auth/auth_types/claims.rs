use super::token::TokenType;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

pub const ACCESS_TOKEN_DURATION: i64 = 15;
pub const REFRESH_TOKEN_DURATION: i64 = 7;

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

impl Claims {
    pub fn new(id: i32, token_type: TokenType) -> Self {
        let exp;
        let permissions;
        match token_type {
            TokenType::AccessToken => {
                exp = usize::try_from(
                    (chrono::Utc::now() + chrono::Duration::minutes(ACCESS_TOKEN_DURATION))
                        .timestamp(),
                )
                .unwrap();
                permissions = Some(vec![format!("READ_{}", id)]);
            }

            TokenType::RefreshToken => {
                exp = usize::try_from(
                    (chrono::Utc::now() + chrono::Duration::minutes(REFRESH_TOKEN_DURATION))
                        .timestamp(),
                )
                .unwrap();
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
