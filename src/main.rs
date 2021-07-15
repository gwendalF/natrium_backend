use crate::config::Config;
use actix_web::{middleware, App, HttpServer};
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
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::NormalizePath::default())
            .service(handlers::temp)
            .service(handlers::hygro)
            .service(handlers::secure_data)
            .service(handlers::create_user)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await?;
    Ok(())
}
