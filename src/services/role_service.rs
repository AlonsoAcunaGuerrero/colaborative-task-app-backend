use actix_web::ResponseError;

use crate::{errors::HttpError, models::Role, repositories::RoleRepository};

pub struct RoleService;

impl RoleService {
    pub async fn get_role_by_name(name: &str) -> Result<Role, HttpError> {
        let found_role = RoleRepository.get_by_name(name).await;

        match found_role {
            Ok(role) => Ok(role),
            Err(e) => Err(HttpError { code: e.status_code(), message: e.to_string() })
        }
    }
}