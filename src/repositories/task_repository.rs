use actix_web::error::ResponseError;
use sqlx::postgres::PgRow;
use sqlx::{PgPool,Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::errors::TaskError;
use crate::models::{State, Task};

use super::{get_pool, Repository};

pub struct TaskRepository;

impl Repository<Task, Uuid> for TaskRepository {
    async fn get_all(&self) -> Result<Vec<Task>, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let list_tasks = sqlx::query("SELECT task_id, task_name, description, 
        creation_date, last_modification_date, editable, state_id, state_name
        FROM col_task_app.get_all_tasks()")
        .map(|row: PgRow| {
            Task {
                task_id: row.get::<Uuid, _>("task_id"),
                task_name: row.get("task_name"),
                description: row.get("description"),
                creation_date: row.get::<DateTime<Utc>, _>("creation_date"),
                last_mod_date: row.get::<DateTime<Utc>, _>("last_modification_date"),
                editable: row.get("editable"),
                state: State {
                    state_id: row.get::<i32, _>("state_id") as u8,
                    state_name: row.get("state_name"),
                }
            }
        }).fetch_all(&pool).await;

        match list_tasks {
            Ok(tasks) => Ok(tasks),
            Err(_) => Err(TaskError::DatabaseError)
        }
    }

    async fn get(&self, id: Uuid) -> Result<Task, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_task = sqlx::query("SELECT task_id, task_name, description, 
        creation_date, last_modification_date, editable, state_id, state_name
        FROM col_task_app.get_all_tasks() WHERE user_id = $1 LIMIT 1")
        .bind(id).map(|row: PgRow| {
            Task {
                task_id: row.get::<Uuid, _>("task_id"),
                task_name: row.get("task_name"),
                description: row.get("description"),
                creation_date: row.get::<DateTime<Utc>, _>("creation_date"),
                last_mod_date: row.get::<DateTime<Utc>, _>("last_modification_date"),
                editable: row.get("editable"),
                state: State {
                    state_id: row.get::<i32, _>("state_id") as u8,
                    state_name: row.get("state_name"),
                }
            }
        }).fetch_optional(&pool).await.map_err(|_| TaskError::DatabaseError)?;

        match found_task {
            Some(t) => Ok(t),
            None => Err(TaskError::NotFoundTaskIDError { id: id })
        }
    }
    
    async fn save(&self, entity: Task) -> Result<Task, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let create = sqlx::query(
            "CALL col_task_app.insert_task($1, $2, $3, $4, $5, $6, $7)")
        .bind(entity.task_id)
        .bind(entity.task_name)
        .bind(entity.description)
        .bind(entity.creation_date)
        .bind(entity.last_mod_date)
        .bind(entity.editable)
        .bind(entity.state.state_id as i32)
        .execute(&pool).await.map_err(|_| TaskError::DatabaseError);

        match create {
            Ok(_) => match TaskRepository.get(entity.task_id).await {
                Ok(t) => Ok(t),
                Err(_) => Err(TaskError::CreateTaskError)
            },
            Err(e) => Err(e)
        }
    }
}

impl TaskRepository {
    pub async fn get_by_name(&self, name: &str) -> Result<Task, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_task = sqlx::query("SELECT task_id, task_name, description, 
        creation_date, last_modification_date, editable, state_id, state_name
        FROM col_task_app.get_all_tasks() WHERE user_id = $1 LIMIT 1")
        .bind(name).map(|row: PgRow| {
            Task {
                task_id: row.get::<Uuid, _>("task_id"),
                task_name: row.get("task_name"),
                description: row.get("description"),
                creation_date: row.get::<DateTime<Utc>, _>("creation_date"),
                last_mod_date: row.get::<DateTime<Utc>, _>("last_modification_date"),
                editable: row.get("editable"),
                state: State {
                    state_id: row.get::<i32, _>("state_id") as u8,
                    state_name: row.get("state_name"),
                }
            }
        }).fetch_optional(&pool).await.map_err(|_| TaskError::DatabaseError)?;

        match found_task {
            Some(t) => Ok(t),
            None => Err(TaskError::NotFoundTaskNameError { name: String::from(name) })
        }
    }
}