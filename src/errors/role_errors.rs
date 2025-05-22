use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};
use uuid::Uuid;

#[derive(Debug, Display, Error)]
pub enum RoleError {
    #[display("Error connecting with the roles database.")]
    DatabaseError,
    #[display("The role with ID {id} couldn't be found.")]
    NotFoundRoleIDError { id: Uuid },
    #[display("The role with name {name} couldn't be found.")]
    NotFoundRoleNameError { name: String },
    #[display("The role can't be created using that data.")]
    CreateRoleError
}

impl error::ResponseError for RoleError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            RoleError::DatabaseError => StatusCode::BAD_REQUEST,
            RoleError::NotFoundRoleIDError { .. } => StatusCode::NOT_FOUND,
            RoleError::NotFoundRoleNameError { .. } => StatusCode::NOT_FOUND,
            RoleError::CreateRoleError => StatusCode::BAD_REQUEST,
        }
    }
}