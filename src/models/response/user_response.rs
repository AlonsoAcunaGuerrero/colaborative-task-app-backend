use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::Role;

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub creation_date: DateTime<Utc>,
    pub last_connection: DateTime<Utc>,
    pub active: bool,
    pub role: Role
}

impl Responder for UserResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok().content_type(ContentType::json()).json(self)
    }
}

