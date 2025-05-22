use actix_web::error::ResponseError;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, Row, PgPool};
use uuid::Uuid;

use crate::{errors::GroupUserError, models::{Group, GroupUser, Role, User}};

use super::get_pool;

pub struct GroupUserRepository;

impl GroupUserRepository{
    pub async fn get_all(&self) -> Result<Vec<GroupUser>, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let list_gu = sqlx::query("SELECT group_id, group_name, 
        group_creation_date, group_description, user_id, full_name, email, username, password_hash, 
        user_creation_date, user_last_connection, user_active, role_id, role_name, role_description 
        FROM col_task_app.get_all_groups_users()")
        .map(|row: PgRow| {
            GroupUser {
                group: Group {
                    group_id: row.get::<Uuid, _>("group_id"),
                    group_name: row.get("group_name"),
                    creation_date: row.get("group_creation_date"),
                    description: row.get("group_description")
                },
                user: User {
                    user_id: row.get::<Uuid, _>("user_id"),
                    full_name: row.get("full_name"),
                    email: row.get("email"),
                    username: row.get("username"),
                    password: row.get("password_hash"),
                    creation_date: row.get::<DateTime<Utc>, _>("user_creation_date"),
                    last_connection: row.get::<DateTime<Utc>, _>("user_last_connection"),
                    active: row.get("user_active"),
                    role: Role {
                        role_id: row.get::<Uuid, _>("role_id"),
                        role_name: row.get("role_name"),
                        description: row.get("role_description")
                    }
                }
            }
        }).fetch_all(&pool).await;

        match list_gu {
            Ok(lgu) => Ok(lgu),
            Err(_) => Err(GroupUserError::DatabaseError)
        }
    }

    pub async fn get_by_ids(&self, user_id: Uuid, group_id: Uuid) -> Result<GroupUser, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_gu = sqlx::query("SELECT group_id, group_name, 
        group_creation_date, group_description, user_id, full_name, email, username, password_hash, 
        user_creation_date, user_last_connection, user_active, role_id, role_name, role_description 
        FROM col_task_app.get_all_groups_users() WHERE user_id = $1 AND group_id = $2 LIMIT 1")
        .bind(user_id).bind(group_id).map(|row: PgRow| {
            GroupUser {
                group: Group {
                    group_id: row.get::<Uuid, _>("group_id"),
                    group_name: row.get("group_name"),
                    creation_date: row.get("group_creation_date"),
                    description: row.get("group_description")
                },
                user: User {
                    user_id: row.get::<Uuid, _>("user_id"),
                    full_name: row.get("full_name"),
                    email: row.get("email"),
                    username: row.get("username"),
                    password: row.get("password_hash"),
                    creation_date: row.get::<DateTime<Utc>, _>("user_creation_date"),
                    last_connection: row.get::<DateTime<Utc>, _>("user_last_connection"),
                    active: row.get("user_active"),
                    role: Role {
                        role_id: row.get::<Uuid, _>("role_id"),
                        role_name: row.get("role_name"),
                        description: row.get("role_description")
                    }
                }
            }
        }).fetch_optional(&pool).await.map_err(|_| GroupUserError::DatabaseError)?;

        match found_gu {
            Some(gu) => Ok(gu),
            None => Err(GroupUserError::NotFoundGroupUserIDError { user_id: user_id, group_id: group_id })
        }
    }

    pub async fn save(&self, entity: GroupUser) -> Result<GroupUser, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let create = sqlx::query(
            "CALL col_task_app.insert_user_into_group($1, $2)")
        .bind(entity.user.user_id)
        .bind(entity.group.group_id)
        .execute(&pool).await.map_err(|_| GroupUserError::DatabaseError);

        match create {
            Ok(_) => match Self.get_by_ids(entity.user.user_id, entity.group.group_id).await {
                Ok(gu) => Ok(gu),
                Err(_) => Err(GroupUserError::AddingUserError)
            },
            Err(e) => Err(e)
        }
    }
}