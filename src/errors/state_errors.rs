use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum StateError {
    #[display("Error connecting with the states Database.")]
    DatabaseError,
    #[display("The state with ID {id} couldn't be found.")]
    NotFoundStateIDError { id: u8 },
    #[display("The state with name {name} couldn't be found.")]
    NotFoundStateNameError { name: String },
    #[display("The state can't be created using that data.")]
    CreateStateError
}

impl error::ResponseError for StateError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            StateError::DatabaseError => StatusCode::BAD_REQUEST,
            StateError::NotFoundStateIDError { .. } => StatusCode::NOT_FOUND,
            StateError::NotFoundStateNameError { .. } => StatusCode::NOT_FOUND,
            StateError::CreateStateError => StatusCode::BAD_REQUEST
        }
    }
}