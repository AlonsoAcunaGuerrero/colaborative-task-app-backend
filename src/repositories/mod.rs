use actix_web::error::ResponseError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, PgPool};
use dotenvy::dotenv;
use std::env;
use std::format;

use crate::errors::DBError;

mod user_repository;
pub use user_repository::UserRepository;

mod role_repository;
pub use role_repository::RoleRepository;

mod state_repository;
pub use state_repository::StateRepository;

mod group_repository;
pub use group_repository::GroupRepository;

mod group_user_repository;
pub use group_user_repository::GroupUserRepository;

mod task_repository;
pub use task_repository::TaskRepository;

pub trait Repository<T, E> {
    async fn get_all(&self) -> Result<Vec<T>, impl ResponseError>;
    async fn get(&self, id: E) -> Result<T, impl ResponseError>;
    async fn save(&self, entity: T) -> Result<T, impl ResponseError>;
    //fn update(&self, entity: T, id: F) -> T;
}


pub async fn get_pool() -> Result<PgPool, DBError> {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").map_err(|_| DBError::NotFoundDatabaseError)?;

    // let schema: String = env::var("DATABASE_SCHEMA").map_err(
    //     |_|
    //     DBError::NotFoundSchemaError
    // )?;

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.map_err(|_| DBError::DatabaseError)?;

    // sqlx::query(format!("SET col_task_app TO {}", schema).as_str())
    // .execute(&pool)
    // .await.map_err(|_| DBError::DatabaseError)?;

    Ok(pool)
}