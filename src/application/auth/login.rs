use chrono::Utc;
use jsonwebtoken::{decode, decode_header, encode, Header, Validation};
use sqlx::PgPool;

use crate::domain::auth::jwt_authentication::{AppKey, Claims, TokenResponse};
use crate::domain::auth::provider;

use crate::domain::auth::value_object::key_identifier::Kid;
use crate::domain::auth::value_object::provider_key::GoogleKeySet;
use crate::domain::core::value_object::ValueObject;
use crate::infrastructure::auth::provider::update_key_set;
use crate::infrastructure::auth::user::check_existing_user;
use crate::AppError;
use crate::Result;

// pub async fn provider_login(
//     provider: provider::AuthProvider,
//     token: &str,
//     pool: &PgPool,
//     google_key: &GoogleKeySet,
//     app_key: &AppKey,
// ) -> Result<TokenResponse> {
//     match provider {
//         provider::AuthProvider::Google(mut key_set) => {
//             if key_set.expiration <= Utc::now().naive_utc() {
//                 update_key_set(&mut key_set, pool).await?;
//             }
//             let header = decode_header(token)?;
//             let kid = Kid::new(
//                 header
//                     .kid
//                     .ok_or_else(|| AppError::TokenError("Missing key identifier".to_owned()))?,
//             );
//             let key = kid.value().unwrap();
//             let decoding_key = &key_set.keys[key];
//             let provider_claims =
//                 decode::<Claims>(token, decoding_key, &Validation::default())?.claims;
//             match provider_claims.iss.as_ref() {
//                 "accounts.google.com" | "https://accounts.google.com" => (),
//                 _ => {
//                     return Err(AppError::PermissionDenied(
//                         "Malicious user detected".to_owned(),
//                     ))
//                 }
//             }
//             let mut user_id = check_existing_user(pool, &provider_claims.sub).await?;
//             if let None = user_id {
//                 // TODO create user
//                 user_id = Some(1);
//             }
//             let user_id = user_id.ok_or_else(|| AppError::ServerError)?;
//             let claims = Claims::new(user_id);
//             let token = encode(&Header::default(), &claims, &app_key.encoding)?;
//             Ok(TokenResponse { token })
//         }
//         _ => unimplemented!(),
//     }
// }
