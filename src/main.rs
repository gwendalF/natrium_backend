use crate::config::Config;
use crate::domain::auth::jwt_authentication;
use crate::errors::Result;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Mutex};

mod application;
mod config;
mod domain;
mod errors;
mod handlers;
mod infrastructure;

#[actix_web::main]
async fn main() -> errors::Result<()> {
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
    let key_map: HashMap<String, DecodingKey> = keys_record
        .iter()
        .map(|record| {
            (
                record.kid.clone(),
                DecodingKey::from_rsa_components(&record.modulus, &record.exponent).into_static(),
            )
        })
        .collect();
    let decoding = DecodingKey::from_secret(config.secret.key.as_bytes()).into_static();
    let encoding = EncodingKey::from_secret(config.secret.key.as_bytes());
    let jwt_key = jwt_authentication::JwtKey { encoding, decoding };
    Ok(HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(jwt_authentication::validator);
        let goolge_key_set = jwt_authentication::GoogleKeySet {
            expiration,
            keys: key_map.clone(),
        };
        App::new()
            .data(db_pool.clone())
            .app_data(web::Data::new(jwt_key.clone()))
            .app_data(web::Data::new(Mutex::new(goolge_key_set.clone())))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "Invalid json",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
            .wrap(middleware::NormalizePath::default())
            .service(handlers::temp)
            .service(handlers::google_login)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .workers(config.workers)
    .run()
    .await?)
}
