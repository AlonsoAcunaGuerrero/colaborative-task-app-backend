use actix_web::error::ResponseError;
use sqlx::postgres::PgRow;
use sqlx::{PgPool,Row};
use uuid::Uuid;

use crate::errors::RoleError;
use crate::models::Role;
use crate::repositories::Repository;
use crate::repositories::get_pool;

pub struct RoleRepository;

impl Repository<Role, Uuid> for RoleRepository {
    async fn get_all(&self) ->  Result<Vec<Role>, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let list_roles = sqlx::query("SELECT role_id, role_name, description 
        FROM col_task_app.roles")
        .map(|row: PgRow| {
            Role {
                role_id: row.get::<Uuid, _>("role_id"),
                role_name: row.get("role_name"),
                description: row.get("description")
            }
        }).fetch_all(&pool).await.map_err(|_| RoleError::DatabaseError);

        match list_roles {
            Ok(roles) => Ok(roles),
            Err(_) => Err(RoleError::DatabaseError)
        }
    }

    async fn get(&self, id: Uuid) -> Result<Role, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_role = sqlx::query_as::<_, Role>("SELECT role_id, role_name, 
        description FROM col_task_app.roles WHERE role_id = $1 LIMIT 1")
        .bind(id).fetch_optional(&pool).await.map_err(|_| RoleError::DatabaseError)?;
        
        match found_role {
            Some(r) => Ok(r),
            None => Err(RoleError::NotFoundRoleIDError { id: id })
        }
    }
    
    async fn save(&self, entity: Role) -> Result<Role, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let create = sqlx::query(
            "CALL col_task_app.insert_role($1, $2, $3)")
        .bind(entity.role_id)
        .bind(entity.role_name)
        .bind(entity.description)
        .execute(&pool).await.map_err(|_| RoleError::DatabaseError);
        
        match create {
            Ok(_) => match RoleRepository.get(entity.role_id).await {
                Ok(r) => Ok(r),
                Err(_) => Err(RoleError::CreateRoleError)
            },
            Err(e) => Err(e)
        }
    }   
}

impl RoleRepository {
    pub async fn get_by_name(&self, name: &str) -> Result<Role, impl ResponseError> {
        let pool: PgPool = get_pool().await.unwrap();

        let found_role = sqlx::query_as::<_, Role>("SELECT role_id, role_name, 
        description FROM col_task_app.roles WHERE role_name = $1 LIMIT 1")
        .bind(name).fetch_optional(&pool).await.map_err(|_| RoleError::DatabaseError)?;

        match found_role {
            Some(r) => Ok(r),
            None => Err(RoleError::NotFoundRoleNameError { name: String::from(name) })
        }
    }
}