use actix_web::error::ResponseError;
use sqlx::postgres::PgRow;
use sqlx::{PgPool,Row};

use crate::errors::StateError;
use crate::models::State;

use super::{get_pool, Repository};

pub struct StateRepository;

impl Repository<State, u8> for StateRepository {
    async fn get_all(&self) -> Result<Vec<State>, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let list_states = sqlx::query("SELECT state_id, state_name
        FROM col_task_app.states")
        .map(|row: PgRow| {
            State {
                state_id: row.get::<i32, _>("state_id") as u8,
                state_name: row.get("state_name")
            }
        }).fetch_all(&pool).await;

        match list_states {
            Ok(states) => Ok(states),
            Err(_) => Err(StateError::DatabaseError)
        }
    }

    async fn get(&self, id: u8) -> Result<State, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_state = sqlx::query("SELECT state_id, state_name
        FROM col_task_app.states WHERE state_id = $1")
        .bind(id as i32).map(|row: PgRow| {
            State {
                state_id: row.get::<i32, _>("state_id") as u8,
                state_name: row.get("state_name")
            }
        }).fetch_optional(&pool).await.map_err(|_| StateError::DatabaseError)?;

        match found_state {
            Some(s) => Ok(s),
            None => Err(StateError::NotFoundStateIDError { id: id })
        }
    }
    
    async fn save(&self, entity: State) -> Result<State, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let create = sqlx::query(
            "CALL col_task_app.insert_state($1)")
        .bind(entity.state_name)
        .execute(&pool).await.map_err(|_| StateError::DatabaseError);
        
        match create {
            Ok(_) => match StateRepository.get(entity.state_id).await {
                Ok(s) => Ok(s),
                Err(_) => Err(StateError::CreateStateError)
            },
            Err(e) => Err(e)
        }
    }
}

impl StateRepository {
    pub async fn get_by_name(&self, name: &str) -> Result<State, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_state = sqlx::query("SELECT state_id, state_name
        FROM col_task_app.states WHERE state_name = $1")
        .bind(name).map(|row: PgRow| {
            State {
                state_id: row.get::<i32, _>("state_id") as u8,
                state_name: row.get("state_name")
            }
        }).fetch_optional(&pool).await.map_err(|_| StateError::DatabaseError)?;

        match found_state {
            Some(s) => Ok(s),
            None => Err(StateError::NotFoundStateNameError { name: String::from(name) })
        }
    }
}