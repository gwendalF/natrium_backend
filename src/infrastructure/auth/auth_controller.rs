use crate::domain::auth::auth_types::credential::{ClearCredential, Credential};
use crate::domain::auth::auth_types::provider::{AuthProvider, ProviderKeySet};
use crate::domain::auth::auth_types::token::Token;
use crate::domain::auth::errors::AuthError;
use crate::domain::auth::ports::IAuthService;
use crate::{AppError, Result};
use actix_web::{error, web, HttpMessage, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Mutex;

pub fn configure<T: 'static + IAuthService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    let google_key_set = Mutex::new(ProviderKeySet {
        expiration: (chrono::Utc::now() - chrono::Duration::hours(1)).naive_utc(),
        keys: HashMap::new(),
    });
    cfg.app_data(service);
    cfg.route(
        "/register_credential/",
        web::post().to(register_credential::<T>),
    );
    cfg.route("/register_google/", web::post().to(register_provider::<T>));
    cfg.route("/login/", web::post().to(login_credential::<T>));
    cfg.route("/login_google/", web::post().to(login_provider::<T>));
    cfg.route("/refresh_token/", web::get().to(refresh_token::<T>));
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _req| {
        error::InternalError::from_response(
            "Invalid json",
            HttpResponse::BadRequest()
                .content_type("application/json")
                .body(format!(r#"{{"error":"{}"#, err)),
        )
        .into()
    }));
    cfg.data(google_key_set);
}

async fn register_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<ClearCredential>,
) -> Result<HttpResponse> {
    let credential = Credential::try_from(body)?;
    let auth_token = service.register_credential(&credential).await?;
    Ok(auth_token.into())
}

async fn login_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<ClearCredential>,
) -> Result<HttpResponse> {
    let auth_token = service.login_credential(&body).await?;
    Ok(auth_token.into())
}

async fn register_provider<T: IAuthService>(
    service: web::Data<T>,
    provider_token: BearerAuth,
    key_set: web::Data<Mutex<ProviderKeySet>>,
) -> Result<HttpResponse> {
    let token = Token(provider_token.token().to_owned());
    let provider = AuthProvider::Google;
    let auth_token = service
        .register_provider(&token, &provider, key_set.get_ref())
        .await?;
    Ok(auth_token.into())
}

async fn login_provider<T: IAuthService>(
    service: web::Data<T>,
    token: BearerAuth,
    key_set: web::Data<Mutex<ProviderKeySet>>,
) -> Result<HttpResponse> {
    let token = Token(token.token().to_owned());
    let provider = AuthProvider::Google;
    let auth_token = service
        .login_provider(&token, &provider, key_set.get_ref())
        .await?;
    Ok(auth_token.into())
}

async fn refresh_token<T: IAuthService>(
    service: web::Data<T>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    if let Some(refresh_token) = req.cookie("refresh_token") {
        let refresh_token = Token(refresh_token.value().to_owned());
        let auth_token = service.refresh_token(&refresh_token).await?;
        Ok(auth_token.into())
    } else {
        Err(AppError::AuthenticationError(AuthError::Token))
    }
}
