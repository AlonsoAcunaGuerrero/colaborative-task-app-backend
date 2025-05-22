use actix_web::{post, web, Responder};

use crate::{
    errors::HttpError, 
    models::CreateTaskRequest, 
    services::TaskService
};

#[post("/new")]
async fn insert_task(req: web::Json<CreateTaskRequest>) -> Result<impl Responder, HttpError> {
    match TaskService::create_task(req.into_inner()).await {
        Ok(task) => Ok(web::Json(task)),
        Err(e) => Err(e)
    }
}