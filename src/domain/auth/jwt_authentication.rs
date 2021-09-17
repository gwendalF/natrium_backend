use crate::AppError;

use actix_web::{dev::ServiceRequest, web};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, errors::ErrorKind, Validation};

use super::{
    auth_types::{claims::Claims, jwt_key::AccessKey},
    errors::AuthError,
};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    if let Some(app_key) = req.app_data::<web::Data<AccessKey>>() {
        let mut validation = Validation {
            iss: Some("natrium".to_owned()),
            ..Validation::default()
        };
        validation.set_audience(&["natrium"]);
        match decode::<Claims>(credentials.token(), &app_key.decoding, &validation) {
            Ok(token) => {
                if let Some(perm) = token.claims.permissions {
                    req.attach(perm);
                }
                Ok(req)
            }
            Err(e) => match e.kind() {
                ErrorKind::ExpiredSignature => Err(AppError::from(AuthError::ExpiredToken).into()),
                _ => Err(AppError::from(AuthError::Token).into()),
            },
        }
    } else {
        Err(AppError::ServerError.into())
    }
}
