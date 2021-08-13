use jsonwebtoken::{encode, Header};
use sqlx::PgPool;

use crate::domain::auth::jwt_authentication::{Claims, TokenResponse};
use crate::domain::auth::provider;
use crate::infrastructure::auth::user::check_existing_user;
use crate::Result;

pub async fn provider_login(
    provider: provider::AuthProvider,
    subject: &str,
    pool: &PgPool,
) -> Result<TokenResponse> {
    if let Some(existing_id) = check_existing_user(pool, subject).await? {
        match provider {
            provider::AuthProvider::Google(key_set) => {
                let claims = Claims::new(existing_id);
                let encoding_key = key_set.keys["key_id_to_get"].clone();
                let token = encode(&Header::default(), &claims, &encoding_key)?;
            }
            _ => unimplemented!(),
        }
    }

    todo!()
}
