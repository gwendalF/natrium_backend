use std::sync::Mutex;

use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, Validation};

use crate::domain::{
    auth::{
        auth_types::{
            claims::{Claims, ProviderClaims, REFRESH_TOKEN_DURATION},
            credential::{ClearCredential, Credential},
            email::EmailAddress,
            jwt_key::{AccessKey, RefreshKey},
            key_identifier::{Kid, KidError},
            password::PasswordError,
            provider::{AuthProvider, ProviderKeySet},
            token::{AuthToken, Token},
        },
        errors::AuthError,
        ports::{IAuthService, TokenRepository, UserRepository},
    },
    AppError,
};
use crate::Result;

pub struct AuthService<T, U> {
    pub repository: T,
    pub application_key: AccessKey,
    pub refresh_key: RefreshKey,
    pub token_repository: U,
}

#[async_trait]
impl<T, U> IAuthService for AuthService<T, U>
where
    T: UserRepository + Send + Sync,
    U: TokenRepository + Send + Sync,
{
    async fn login_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<AuthToken> {
        let provider_claims =
            decode_provider(provider, provider_token, &self.repository, key_set).await?;
        let user_id = self
            .repository
            .check_existing_user_provider(&provider_claims.sub)
            .await?;
        let token = AuthToken::new(
            user_id,
            &self.application_key.encoding,
            &self.refresh_key.encoding,
        )?;
        let expiration = second_usize();
        self.token_repository
            .save_token(user_id, &token.refresh_token, expiration)
            .await?;
        Ok(token)
    }

    async fn login_credential(&self, credential: &ClearCredential) -> Result<AuthToken> {
        let user_email =
            EmailAddress::new(credential.email.clone()).expect("error management really needed");
        let user_id = self
            .repository
            .check_existing_user_email(&user_email)
            .await?;
        let user_hash = self.repository.hash(&user_email).await?;
        let password_valid = argon2::verify_encoded(&user_hash, &credential.password.as_bytes())?;
        if password_valid {
            let token = AuthToken::new(
                user_id,
                &self.application_key.encoding,
                &self.refresh_key.encoding,
            )?;
            let expiration =
                chrono::Duration::minutes(REFRESH_TOKEN_DURATION).num_seconds() as usize;
            self.token_repository
                .save_token(user_id, &token.refresh_token, expiration)
                .await?;
            Ok(token)
        } else {
            Err(AuthError::Password(PasswordError::InvalidPassword).into())
        }
    }

    async fn register_credential(&self, credential: &Credential) -> Result<AuthToken> {
        let user_id = self.repository.create_user_credential(credential).await?;
        let token = AuthToken::new(
            user_id,
            &self.application_key.encoding,
            &self.refresh_key.encoding,
        )?;
        let expiration = second_usize();
        self.token_repository
            .save_token(user_id, &token.refresh_token, expiration)
            .await?;
        Ok(token)
    }

    async fn register_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<AuthToken> {
        let claims = decode_provider(provider, provider_token, &self.repository, key_set).await?;
        let user_id = self
            .repository
            .create_user_subject(
                &claims.sub,
                &EmailAddress::new(claims.email).expect("Error to be managed"),
            )
            .await?;
        let token = AuthToken::new(
            user_id,
            &self.application_key.encoding,
            &self.refresh_key.encoding,
        )?;
        self.token_repository
            .save_token(user_id, &token.refresh_token, second_usize())
            .await?;
        Ok(token)
    }

    async fn refresh_token(&self, refresh_token: &Token) -> Result<AuthToken> {
        let mut validation = Validation {
            iss: Some("natrium".to_owned()),
            ..Validation::default()
        };
        validation.set_audience(&["natrium"]);
        let refresh_claims =
            decode::<Claims>(&refresh_token.0, &self.refresh_key.decoding, &validation)?.claims;
        let user_id = refresh_claims
            .sub
            .parse::<i32>()
            .map_err(|_| AppError::from(AuthError::Token))?;
        if refresh_claims.permissions == Some(vec![format!("ACCESS_TOKEN_{}", user_id)]) {
            self.token_repository
                .check_existing_token(user_id, refresh_token)
                .await?;
            let auth_token = AuthToken::new(
                user_id,
                &self.application_key.encoding,
                &self.refresh_key.encoding,
            )?;
            self.token_repository
                .save_token(user_id, &auth_token.refresh_token, second_usize())
                .await?;
            Ok(auth_token)
        } else {
            Err(AuthError::Token.into())
        }
    }
}

async fn decode_provider<T>(
    provider: &AuthProvider,
    token: &Token,
    repository: &T,
    key_set: &Mutex<ProviderKeySet>,
) -> Result<ProviderClaims>
where
    T: UserRepository + Sync + Send,
{
    let header = decode_header(&token.0)?;
    match provider {
        AuthProvider::Facebook => unimplemented!(),
        AuthProvider::Google => {
            let expiration;
            {
                let key_set_guard = key_set.lock().expect("Lock error");
                expiration = key_set_guard.expiration;
            }

            if expiration <= chrono::Utc::now().naive_utc() {
                repository.update_key_set(key_set).await?;
            }
            let kid = Kid::new(header.kid.ok_or(AuthError::Kid(KidError::InvalidKid))?)
                .map_err(|_| AuthError::Kid(KidError::InvalidKid))?;
            let key_set = key_set.lock().expect("Lock error");
            let decoding_key = &key_set.keys[&kid];
            let validation = Validation {
                algorithms: vec![jsonwebtoken::Algorithm::RS256],
                ..Validation::default()
            };
            let provider_claims =
                decode::<ProviderClaims>(&token.0, decoding_key, &validation)?.claims;
            match provider_claims.iss.as_ref() {
                "accounts.google.com" | "https://accounts.google.com" => Ok(provider_claims),
                _ => Err(AuthError::Token.into()),
            }
        }
    }
}

fn second_usize() -> usize {
    chrono::Duration::days(REFRESH_TOKEN_DURATION).num_seconds() as usize
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;
    use crate::domain::auth::{
        auth_types::salt::Salt,
        ports::{MockTokenRepository, MockUserRepository},
    };
    use jsonwebtoken::{DecodingKey, EncodingKey};
    use mockall::predicate::*;

    #[actix_web::main]
    #[test]
    async fn test_credential_login_success() {
        let credential = ClearCredential {
            email: "test@example.com".to_owned(),
            password: "password".to_owned(),
        };
        let salt = Salt::try_from("7aMCdqknwnk3gBCFW7G9xNVi5kb2LGR6".to_owned()).unwrap();
        let user_hash = argon2::hash_encoded(
            &credential.password.as_bytes(),
            salt.value().as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap();
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_check_existing_user_email()
            .returning(|_| Ok(1));

        let mut token_repository = MockTokenRepository::new();
        user_repo
            .expect_hash()
            .returning(move |_| Ok(user_hash.clone()));
        token_repository
            .expect_save_token()
            .returning(|_, _, _| Ok(()));
        let encoding = EncodingKey::from_secret("secret_key".as_bytes());
        let decoding = DecodingKey::from_secret("secret_key".as_bytes()).into_static();
        let application_key = AccessKey {
            encoding: encoding.clone(),
            decoding: decoding.clone(),
        };
        let refresh_key = RefreshKey {
            encoding: encoding.clone(),
            decoding,
        };
        let sut = AuthService {
            repository: user_repo,
            application_key,
            refresh_key,
            token_repository,
        };
        let actual = sut.login_credential(&credential).await.unwrap();
        let expected = AuthToken::new(1, &encoding, &encoding).unwrap();
        assert_eq!(actual, expected);
    }

    #[actix_web::main]
    #[test]
    async fn test_login_google() {
        let user_id = 1;
    }
}
