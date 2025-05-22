use actix_web::{post, web};

use crate::errors::HttpError;
use crate::services::AuthService;
use crate::models::{LoginRequest, LoginResponse};

#[post("/login")]
async fn login(login_req: web::Json<LoginRequest>) -> Result<LoginResponse, HttpError> {
    let login_data = login_req.into_inner();

    match AuthService::login(login_data).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}