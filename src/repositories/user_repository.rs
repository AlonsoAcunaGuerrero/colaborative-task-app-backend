use actix_web::error::ResponseError;
use sqlx::postgres::PgRow;
use sqlx::{PgPool,Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::errors::UserError;
use crate::models::{User, Role};
use crate::repositories::Repository;
use crate::repositories::get_pool;

pub struct UserRepository;

impl Repository<User, Uuid> for UserRepository {
    async fn get_all(&self) -> Result<Vec<User>, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let list_users = sqlx::query("SELECT user_id, full_name, email, username, password_hash, creation_date, 
               last_connection, active, role_id, role_name, role_description
        FROM col_task_app.get_all_users()")
        .map(|row: PgRow| {
            User {
                user_id: row.get::<Uuid, _>("user_id"),
                full_name: row.get("full_name"),
                email: row.get("email"),
                username: row.get("username"),
                password: row.get("password_hash"),
                creation_date: row.get::<DateTime<Utc>, _>("creation_date"),
                last_connection: row.get::<DateTime<Utc>, _>("last_connection"),
                active: row.get("active"),
                role: Role {
                    role_id: row.get::<Uuid, _>("role_id"),
                    role_name: row.get("role_name"),
                    description: row.get("role_description")
                }
            }
        }).fetch_all(&pool).await;

        match list_users {
            Ok(users) => Ok(users),
            Err(_) => Err(UserError::DatabaseError)
        }
    }

    async fn get(&self, id: Uuid) -> Result<User, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_user = sqlx::query("SELECT user_id, full_name, email, username, password_hash, 
        creation_date, last_connection, active, role_id, role_name, role_description FROM col_task_app.get_all_users()
        WHERE user_id = $1 LIMIT 1")
        .bind(id).map(|row: PgRow| {
            User {
                user_id: row.get::<Uuid, _>("user_id"),
                full_name: row.get("full_name"),
                email: row.get("email"),
                username: row.get("username"),
                password: row.get("password_hash"),
                creation_date: row.get::<DateTime<Utc>, _>("creation_date"),
                last_connection: row.get::<DateTime<Utc>, _>("last_connection"),
                active: row.get("active"),
                role: Role {
                    role_id: row.get::<Uuid, _>("role_id"),
                    role_name: row.get("role_name"),
                    description: row.get("role_description")
                }
            }
        }).fetch_optional(&pool).await.map_err(|_| UserError::DatabaseError)?;

        match found_user {
            Some(u) => Ok(u),
            None => Err(UserError::NotFoundUserIDError { id: id })
        }
    }

    async fn save(&self, entity: User) -> Result<User, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let create = sqlx::query(
            "CALL col_task_app.insert_user($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(entity.user_id)
        .bind(entity.full_name)
        .bind(entity.email)
        .bind(entity.password)
        .bind(entity.creation_date)
        .bind(entity.last_connection)
        .bind(entity.active)
        .bind(entity.role.role_id)
        .bind(entity.username)
        .execute(&pool).await.map_err(|_| UserError::DatabaseError);

        match create {
            Ok(_) => {
                match UserRepository.get(entity.user_id).await {
                    Ok(u) => Ok(u),
                    Err(_) => Err(UserError::CreateUserError)
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl UserRepository {
    pub async fn get_by_email(&self, email: &str) -> Result<User, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_user = sqlx::query("SELECT user_id, full_name, email, username, password_hash, creation_date, 
               last_connection, active, role_id, role_name, role_description
        FROM col_task_app.get_all_users() WHERE email = $1 LIMIT 1")
        .bind(email).map(|row: PgRow| {
            User {
                user_id: row.get::<Uuid, _>("user_id"),
                full_name: row.get("full_name"),
                email: row.get("email"),
                username: row.get("username"),
                password: row.get("password_hash"),
                creation_date: row.get::<DateTime<Utc>, _>("creation_date"),
                last_connection: row.get::<DateTime<Utc>, _>("last_connection"),
                active: row.get("active"),
                role: Role {
                    role_id: row.get::<Uuid, _>("role_id"),
                    role_name: row.get("role_name"),
                    description: row.get("role_description")
                }
            }
        }).fetch_optional(&pool).await.map_err(|_| UserError::DatabaseError)?;

        match found_user {
            Some(u) => Ok(u),
            None => Err(UserError::NotFoundUserEmailError { email: String::from(email) })
        }
    }
}