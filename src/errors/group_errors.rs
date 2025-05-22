use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};
use uuid::Uuid;

#[derive(Debug, Display, Error)]
pub enum GroupError {
    #[display("Error connecting with the groups database.")]
    DatabaseError,
    #[display("The group with ID {id} couldn't be found.")]
    NotFoundGroupIDError { id: Uuid },
    #[display("The group with name {name} couldn't be found.")]
    NotFoundGroupNameError { name: String },
    #[display("The group can't be created using that data.")]
    CreateGroupError
}

impl error::ResponseError for GroupError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            GroupError::DatabaseError => StatusCode::BAD_REQUEST,
            GroupError::NotFoundGroupIDError { .. } => StatusCode::NOT_FOUND,
            GroupError::NotFoundGroupNameError { .. } => StatusCode::NOT_FOUND,
            GroupError::CreateGroupError => StatusCode::BAD_REQUEST,
        }
    }
}