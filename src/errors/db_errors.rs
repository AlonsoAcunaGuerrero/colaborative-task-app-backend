use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum DBError {
    #[display("Error connecting with the database.")]
    DatabaseError,
    #[display("Error connecting with the database, verify the Schema.")]
    NotFoundSchemaError,
    #[display("Error connecting with the database, verify the Database URL.")]
    NotFoundDatabaseError
}

impl error::ResponseError for DBError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            DBError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            DBError::NotFoundSchemaError => StatusCode::INTERNAL_SERVER_ERROR,
            DBError::NotFoundDatabaseError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}