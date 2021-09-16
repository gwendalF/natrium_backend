use std::sync::{Arc, Mutex};

use crate::domain::auth::errors::AuthError;
use crate::domain::auth::ports::{AuthToken, Token, TokenRepository};
use crate::Result;
use async_trait::async_trait;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct TokenRepositoryImpl {
    pub token_repo: redis::aio::MultiplexedConnection,
}

#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
    async fn save_token(
        &self,
        user_id: i32,
        refresh_token: &Token,
        expiration: usize,
    ) -> Result<()> {
        let mut conn = self.token_repo.clone();
        let _: () = conn
            .set_ex(user_id, &refresh_token.0, expiration)
            .await
            .expect("Cannot set");

        Ok(())
    }

    async fn check_existing_token(&self, user_id: i32, refresh_token: &Token) -> Result<()> {
        let mut conn = self.token_repo.clone();
        println!("user_id: {}\n", user_id);
        let response: Option<String> = conn.get(user_id).await.expect("err");
        println!("{:?}", response);
        if let Some(redis_token) = response {
            if redis_token == refresh_token.0 {
                Ok(())
            } else {
                Err(AuthError::Token.into())
            }
        } else {
            Err(AuthError::ExpiredToken.into())
        }
    }
}
