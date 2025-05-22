use actix_web::{dev::ServiceRequest, error, http::StatusCode, web, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{errors::HttpError, services::{AuthService, TokenType}};

pub async fn jwt_validator(req: ServiceRequest, credentials: Option<BearerAuth>) 
-> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest(web::Json(HttpError{
            code: StatusCode::BAD_REQUEST,
            message: String::from("Auth Token not especified.")
        })), req));
    };

    let token = credentials.token().to_owned();
    let res = AuthService::read_token(token).await;

    match res {
        Ok(claims) => {
            match claims.tt {
                TokenType::ACCESS => Ok(req),
                TokenType::REFRESH => Err((error::ErrorForbidden("Access denied."), req))
            }
        },
        Err(_) => Err((error::ErrorForbidden("Access denied."), req))
    }
}