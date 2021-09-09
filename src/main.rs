use crate::config::Config;
use crate::domain::auth::jwt_authentication;
use actix_web::{middleware, web, App, HttpServer};
use application::auth::auth_service_impl::AuthService;
use domain::auth::auth_types::key_identifier::Kid;
use domain::auth::auth_types::provider::AuthProvider;
use domain::auth::ports::ProviderKeySet;
use domain::AppError;
use domain::Result;
use infrastructure::auth::postgres_repo::UserRepositoryImpl;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Mutex;

mod application;
mod config;
mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    let database_url = std::env::var("DATABASE_URL")?;
    let db_pool = PgPool::connect(&database_url).await?;
    let keys_record = sqlx::query!(
        "SELECT modulus, exponent, kid, expiration FROM token_key WHERE provider='google'"
    )
    .fetch_all(&db_pool)
    .await?;
    let expiration = keys_record[0].expiration;
    let key_map: HashMap<Kid, DecodingKey> = keys_record
        .into_iter()
        .map(|record| {
            Ok((
                Kid::new(record.kid)?,
                DecodingKey::from_rsa_components(&record.modulus, &record.exponent).into_static(),
            ))
        })
        .collect::<Result<HashMap<Kid, DecodingKey>>>()?;
    let decoding = DecodingKey::from_secret(config.secret.key.as_bytes()).into_static();
    let encoding = EncodingKey::from_secret(config.secret.key.as_bytes());
    let jwt_key = jwt_authentication::AppKey { encoding, decoding };
    Ok(HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(jwt_authentication::validator);
        let google_key_set = Mutex::new(ProviderKeySet {
            expiration,
            keys: key_map.clone(),
        });
        App::new()
            .configure(|cfg| {
                let service = AuthService {
                    repository: UserRepositoryImpl {
                        repo: db_pool.clone(),
                    },
                    application_key: jwt_key.clone(),
                };
                infrastructure::auth::auth_controller::configure(web::Data::new(service), cfg)
            })
            .wrap(middleware::NormalizePath::default())
            .app_data(web::Data::new(google_key_set).clone())
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .workers(config.workers)
    .run()
    .await?)
}
