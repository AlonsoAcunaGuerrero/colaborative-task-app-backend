use actix_web::{http::StatusCode, ResponseError, Result};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    errors::HttpError, 
    models::{
        CreateUserRequest, UserResponse, User
    }, 
    repositories::{
        Repository, UserRepository
    }
};
use super::RoleService;
use super::AuthService;

pub struct UserService;

impl UserService{
    pub async fn get_all_users() -> Result<Vec<User>, HttpError> {
        let list_users = UserRepository.get_all().await.map_err(|e| e);

        match list_users {
            Ok(users) => Ok(users),
            Err(e) => Err(HttpError { code: e.status_code(), message: e.to_string() })
        }
    }

    pub async fn get_user_by_id(id: Uuid) -> Result<User, HttpError> {
        let req_user = UserRepository.get(id).await;
        
        match req_user {
            Ok(user) => Ok(user),
            Err(e) => Err(HttpError { code: e.status_code(), message: e.to_string() })
        } 
    }

    pub async fn get_user_by_email(email: &str) -> Result<User, HttpError> {
        if !email.contains("@") {
            return Err(HttpError { 
                code: StatusCode::BAD_REQUEST,
                message: String::from("The email has been written incorrectly.")
            });
        }
        
        let req_user = UserRepository.get_by_email(&email).await;
        
        match req_user {
            Ok(user) => Ok(user),
            Err(e) => Err(HttpError { code: e.status_code(), message: e.to_string() })
        } 
    }

    pub async fn create_user(request: CreateUserRequest) -> Result<UserResponse, HttpError> {
        if request.role.contains("Admin") {
            return Err(HttpError{
                code: StatusCode::BAD_REQUEST,
                message: String::from("You can't create a Admin user.")
            });
        }

        let is_email_used = match UserService::get_user_by_email(request.email.as_str())
        .await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.status_code().eq(&StatusCode::NOT_FOUND) {Ok(false)} else {Err(e)}
            }
        }.map_err(|e| e)?;

        if is_email_used {
            return Err(HttpError { 
                code: StatusCode::BAD_REQUEST,
                message: String::from("The email is already in use.")
            });
        }

        let role = RoleService::get_role_by_name(request.role.as_str()).await.map_err(|e| e)?;

        let password_hash = AuthService::encrypt_password(request.password).await?;

        let user: User = User {
            user_id: Uuid::new_v4(),
            full_name: request.full_name,
            email: request.email,
            username: request.username,
            password: password_hash,
            creation_date: Utc::now(),
            last_connection: Utc::now(),
            active: true,
            role: role
        };

        let save_user =  UserRepository.save(user).await;

        match save_user {
            Ok(u) => Ok(UserResponse{
                user_id: u.user_id,
                full_name: u.full_name,
                email: u.email,
                username: u.username,
                creation_date: u.creation_date,
                last_connection: u.last_connection,
                active: u.active,
                role: u.role
            }),
            Err(e) => Err(HttpError { code: e.status_code(), message: e.to_string() }) 
        }
    }
}

