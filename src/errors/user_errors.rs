use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};
use uuid::Uuid;

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display("Error connecting with the users database.")]
    DatabaseError,
    #[display("The user with ID {id} couldn't be found.")]
    NotFoundUserIDError { id: Uuid },
    #[display("The user with email {email} couldn't be found.")]
    NotFoundUserEmailError { email: String },
    #[display("The user can't be created using that data.")]
    CreateUserError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::DatabaseError => StatusCode::BAD_REQUEST,
            UserError::NotFoundUserIDError { .. } => StatusCode::NOT_FOUND,
            UserError::NotFoundUserEmailError { .. } => StatusCode::NOT_FOUND,
            UserError::CreateUserError => StatusCode::BAD_REQUEST,
        }
    }
}