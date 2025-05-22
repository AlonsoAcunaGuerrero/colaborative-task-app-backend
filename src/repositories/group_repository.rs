use actix_web::error::ResponseError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::GroupError, models::Group};

use super::{get_pool, Repository};

pub struct GroupRepository;

impl Repository<Group, Uuid> for GroupRepository {
    async fn get_all(&self) -> Result<Vec<Group>, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let list_groups = sqlx::query_as::<_, Group>("SELECT group_id, group_name, 
        creation_date, description FROM col_task_app.groups")
        .fetch_all(&pool).await.map_err(|_| GroupError::DatabaseError);

        match list_groups {
            Ok(groups) => Ok(groups),
            Err(e) => Err(e)
        }
    }

    async fn get(&self, id: Uuid) -> Result<Group, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_group = sqlx::query_as::<_, Group>("SELECT group_id, group_name, 
        creation_date, description FROM col_task_app.groups WHERE group_id = $1 LIMIT 1")
        .bind(id).fetch_optional(&pool).await.map_err(|_| GroupError::DatabaseError)?;

        match found_group {
            Some(g) => Ok(g),
            None => Err(GroupError::NotFoundGroupIDError { id: id })
        }
    }

    async fn save(&self, entity: Group) -> Result<Group, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let create = sqlx::query(
            "CALL col_task_app.insert_group($1, $2, $3, $4)")
        .bind(entity.group_id)
        .bind(entity.group_name)
        .bind(entity.creation_date)
        .bind(entity.description)
        .execute(&pool).await.map_err(|_| GroupError::DatabaseError);

        match create {
            Ok(_) => match GroupRepository.get(entity.group_id).await {
                Ok(g) => Ok(g),
                Err(_) => Err(GroupError::CreateGroupError)
            },
            Err(e) => Err(e)
        }
    }
}

impl GroupRepository {
    pub async fn get_by_name(&self, name: &str) -> Result<Group, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_task = sqlx::query_as::<_, Group>("SELECT group_id, group_name, 
        creation_date, description FROM col_task_app.groups WHERE group_name = $1 LIMIT 1")
        .bind(name).fetch_optional(&pool).await.map_err(|_| GroupError::DatabaseError)?;

        match found_task {
            Some(t) => Ok(t),
            None => Err(GroupError::NotFoundGroupNameError { name: String::from(name) })
        }
    }
}