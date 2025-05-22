use actix_web::{get, post, web, Responder};
use uuid::Uuid;

use crate::errors::HttpError;
use crate::services::UserService;
use crate::models::{CreateUserRequest, UserResponse};

#[get("/show")]
async fn get_all_users() -> Result<impl Responder, HttpError> {
    let list_users = UserService::get_all_users().await;

    match list_users {
        Ok(users) => Ok(web::Json(users)),
        Err(e) => Err(e)
    }
}

#[get("/show/{id}")]
async fn get_user_by_id(path: web::Path<Uuid>) -> Result<impl Responder, HttpError> {
    let user_id: Uuid = path.into_inner();

    match UserService::get_user_by_id(user_id).await {
        Ok(user) => Ok(web::Json(user)),
        Err(e) => Err(e)
    }
}

#[post("/new")]
async fn insert_user(req: web::Json<CreateUserRequest>) -> Result<UserResponse, HttpError> {
    match UserService::create_user(req.into_inner()).await {
        Ok(u) => Ok(u),
        Err(e) => Err(e)
    }
}