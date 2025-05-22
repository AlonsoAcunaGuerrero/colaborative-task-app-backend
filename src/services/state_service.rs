use actix_web::ResponseError;

use crate::{errors::HttpError, models::State, repositories::StateRepository};

pub struct StateService;

impl StateService {
    pub async fn get_state_by_name(name: &str) -> Result<State, HttpError> {
        let found_state = StateRepository.get_by_name(name).await;

        match found_state {
            Ok(state) => Ok(state),
            Err(e) => Err(HttpError { code: e.status_code(), message: e.to_string() })
        }
    }
}