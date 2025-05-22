use actix_web::{http::StatusCode, ResponseError};
use chrono::Utc;
use uuid::Uuid;

use crate::{errors::HttpError, models::{CreateGroupRequest, Group}, repositories::{GroupRepository, Repository}};

pub struct GroupService;

impl GroupService {

    async fn get_group_by_name(name: &str) -> Result<Group, HttpError> {
        let req_group = GroupRepository.get_by_name(name).await;

        match req_group {
            Ok(g) => Ok(g),
            Err(e) => Err(HttpError{code: e.status_code(), message: e.to_string()})
        }
    }

    pub async fn create_group(request: CreateGroupRequest) -> Result<Group, HttpError> {
        
        let group_name_exist = match GroupService::get_group_by_name(request.group_name.as_str()).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.status_code().eq(&StatusCode::NOT_FOUND) {Ok(false)} else {Err(e)}
            }
        }.map_err(|e| e)?;

        if group_name_exist {
            return Err(HttpError{
                code: StatusCode::BAD_REQUEST,
                message: String::from("A group with that name already exist.")
            });
        }

        let description = match request.description {
            None => String::from(""),
            Some(text) => text
        };
        
        let group = Group {
            group_id: Uuid::new_v4(),
            group_name: request.group_name,
            creation_date: Utc::now(),
            description
        };

        let save_group = GroupRepository.save(group).await;

        match save_group {
            Ok(g) => Ok(g),
            Err(e) => Err(HttpError{code: e.status_code(), message: e.to_string()})
        }
    }
}