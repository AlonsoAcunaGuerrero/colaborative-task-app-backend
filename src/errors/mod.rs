use derive_more::derive::{Display, Error};
use serde_derive::Serialize;
use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};

#[derive(Debug, Display, Error)]
#[display("Error: {message}")]
pub struct HttpError {
    pub code: StatusCode,
    pub message: String
}

#[derive(Serialize)]
struct HttpErrorNorm {
    pub code: u16,
    pub message: String
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        let norm: HttpErrorNorm = HttpErrorNorm{ 
            code: self.code.as_u16(), 
            message: self.message.clone() 
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(norm)
    }

    fn status_code(&self) -> StatusCode {
        self.code
    }
}

mod auth_errors;
pub use auth_errors::AuthError;

mod user_errors;
pub use user_errors::UserError;

mod role_errors;
pub use role_errors::RoleError;

mod group_errors;
pub use group_errors::GroupError;

mod group_user_errors;
pub use group_user_errors::GroupUserError;

mod state_errors;
pub use state_errors::StateError;

mod task_errors;
pub use task_errors::TaskError;

mod db_errors;
pub use db_errors::DBError;
