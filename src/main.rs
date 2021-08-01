use std::{collections::HashMap, sync::Mutex};

use crate::{auth::jwt_authentication::GoogleKeySet, config::Config};
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::DecodingKey;
use sqlx::PgPool;

mod auth;
mod config;
mod data;
mod errors;
mod handlers;

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
    Ok(HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth::jwt_authentication::validator);
        let goolge_key_set = GoogleKeySet {
            expiration,
            keys: key_map.clone(),
        };
        App::new()
            .data(db_pool.clone())
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
            .service(handlers::secure_data)
            .service(handlers::create_user)
            .service(handlers::temp)
            .service(
                web::scope("/{user_id}")
                    .wrap(auth)
                    .service(handlers::hubs_list)
                    .service(handlers::rack_temperatures)
                    .service(handlers::add_temperature)
                    .service(handlers::user_detail),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .workers(config.workers)
    .run()
    .await?)
}
