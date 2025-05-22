use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};
use uuid::Uuid;

#[derive(Debug, Display, Error)]
pub enum TaskError {
    #[display("Error connecting with the tasks database.")]
    DatabaseError,
    #[display("The task with ID {id} couldn't be found.")]
    NotFoundTaskIDError { id: Uuid },
    #[display("The task with name {name} couldn't be found.")]
    NotFoundTaskNameError { name: String },
    #[display("The task can't be created using that data.")]
    CreateTaskError
}

impl error::ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            TaskError::DatabaseError => StatusCode::BAD_REQUEST,
            TaskError::NotFoundTaskIDError { .. } => StatusCode::NOT_FOUND,
            TaskError::NotFoundTaskNameError { .. } => StatusCode::NOT_FOUND,
            TaskError::CreateTaskError => StatusCode::BAD_REQUEST
        }
    }
}