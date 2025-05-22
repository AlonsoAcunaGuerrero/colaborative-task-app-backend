use actix_web::ResponseError;
use chrono::Utc;
use uuid::Uuid;

use crate::{errors::HttpError, models::{CreateTaskRequest, Task}, repositories::{Repository, TaskRepository}};

use super::StateService;

pub struct TaskService;

impl TaskService {
    pub async fn create_task(request: CreateTaskRequest) -> Result<Task, HttpError> {
        
        let description = match request.description {
            None => String::from(""),
            Some(d) => d
        };

        let is_editable = match request.editable {
            None => true,
            Some(e) => e
        };
        
        let state = match request.state {
            None => match StateService::get_state_by_name("Pending").await {
                Ok(t) => Ok(t),
                Err(e) => Err(e)
            },
            Some(e) => match StateService::get_state_by_name(e.as_str()).await {
                Ok(t) => Ok(t),
                Err(e) => Err(e)
            }
        }?;

        let task = Task {
            task_id: Uuid::new_v4(),
            task_name: request.task_name,
            description: description,
            creation_date: Utc::now(),
            last_mod_date: Utc::now(),
            editable: is_editable,
            state: state
        };

        let save_task = TaskRepository.save(task).await;

        match save_task {
            Ok(task) => Ok(task),
            Err(e) => Err(HttpError { code: e.status_code(), message: e.to_string() })
        }
    }
}