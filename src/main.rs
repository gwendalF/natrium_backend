use crate::config::Config;
use crate::domain::auth::jwt_authentication;
use actix_web::{middleware, web, App, HttpServer};
use application::auth::auth_service_impl::AuthService;
use domain::AppError;
use domain::Result;
use infrastructure::auth::postgres_repo::UserRepositoryImpl;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

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
    let decoding = DecodingKey::from_secret(config.secret.key.as_bytes()).into_static();
    let encoding = EncodingKey::from_secret(config.secret.key.as_bytes());
    let jwt_key = jwt_authentication::AppKey { encoding, decoding };
    let decoding_refresh =
        DecodingKey::from_secret(config.secret.refresh_key.as_bytes()).into_static();
    let encoding_refresh = EncodingKey::from_secret(config.secret.refresh_key.as_bytes());
    let refresh_key = jwt_authentication::RefreshKey {
        encoding: encoding_refresh,
        decoding: decoding_refresh,
    };
    Ok(HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(jwt_authentication::validator);
        App::new()
            .configure(|cfg| {
                let service = AuthService {
                    repository: UserRepositoryImpl {
                        repo: db_pool.clone(),
                    },
                    application_key: jwt_key.clone(),
                    refresh_key: refresh_key.clone(),
                };
                infrastructure::auth::auth_controller::configure(web::Data::new(service), cfg)
            })
            .wrap(middleware::NormalizePath::default())
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .workers(config.workers)
    .run()
    .await?)
}
