use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

mod request;
pub use request::{
    CreateUserRequest,
    LoginRequest, 
    CreateTaskRequest, 
    CreateGroupRequest
};

mod response;
pub use response::UserResponse;
pub use response::LoginResponse;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Role {
    pub role_id: Uuid,
    pub role_name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Permission {
    pub permission_id: Uuid,
    pub role: Role,
    pub action: String
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct State {
    pub state_id: u8,
    pub state_name: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub creation_date: DateTime<Utc>,
    pub last_connection: DateTime<Utc>,
    pub active: bool,
    pub role: Role
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Task {
    pub task_id: Uuid,
    pub task_name: String,
    pub description: String,
    pub creation_date: DateTime<Utc>,
    pub last_mod_date: DateTime<Utc>,
    pub editable: bool,
    pub state: State
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Group {
    pub group_id: Uuid,
    pub group_name: String,
    pub creation_date: DateTime<Utc>,
    pub description: String
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Activity {
    pub activity_id: Uuid,
    pub activity_name: String,
    pub description: String,
    pub mod_date: DateTime<Utc>,
    pub user: User,
    pub task: Task
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct GroupUser {
    pub group: Group,
    pub user: User
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct GroupTask {
    pub group: Group,
    pub task: Task
}