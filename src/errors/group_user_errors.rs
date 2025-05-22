use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};
use uuid::Uuid;

#[derive(Debug, Display, Error)]
pub enum GroupUserError {
    #[display("Error connecting with the users and groups database.")]
    DatabaseError,
    #[display("The user with ID {user_id} couldn't be found in group with ID {group_id}.")]
    NotFoundGroupUserIDError { user_id: Uuid, group_id: Uuid },
    #[display("Error adding the user to the group.")]
    AddingUserError,
    // #[display("The group with ID {id} couldn't be found.")]
    // NotFoundGroupIDError { id: Uuid },
    // #[display("The group with name {name} couldn't be found.")]
    // NotFoundGroupNameError { name: String },
    // #[display("The group can't be created using that data.")]
    // CreateGroupError
}

impl error::ResponseError for GroupUserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            GroupUserError::DatabaseError => StatusCode::BAD_REQUEST,
            GroupUserError::NotFoundGroupUserIDError { .. } => StatusCode::NOT_FOUND,
            GroupUserError::AddingUserError => StatusCode::BAD_REQUEST
        }
    }
}
