use crate::domain::auth::auth_types::credential::{ClearCredential, Credential};
use crate::domain::auth::auth_types::provider::AuthProvider;
use crate::domain::auth::ports::{IAuthService, ProviderKeySet, Token};
use crate::Result;
use actix_web::{web, HttpRequest};

pub fn configure<T: 'static + IAuthService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
}

async fn register_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<Credential>,
) -> Result<web::Json<Token>> {
    Ok(web::Json(service.register_credential(&body).await?))
}

async fn login_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<ClearCredential>,
) -> Result<web::Json<Token>> {
    Ok(web::Json(service.login_credential(&body).await?))
}

async fn register_provider<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<Token>,
    req: HttpRequest,
) -> Result<web::Json<Token>> {
    let key_set = req.app_data::<ProviderKeySet>().expect("Todo later");
    let provider = AuthProvider::Google(key_set.clone());
    Ok(web::Json(service.register_provider(&body, provider).await?))
}

async fn login_provider<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<Token>,
    req: HttpRequest,
) -> Result<web::Json<Token>> {
    let key_set = req.app_data::<ProviderKeySet>().expect("Todo later");
    let provider = AuthProvider::Google(key_set.clone());
    Ok(web::Json(service.login_provider(&body, provider).await?))
}