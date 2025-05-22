use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthError {
    #[display("Error encrypting the user's password.")]
    EncryptionError,
    #[display("Error, the credentials aren't valid.")]
    NotValidDataError
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            AuthError::EncryptionError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::NotValidDataError => StatusCode::UNAUTHORIZED
        }
    }
}