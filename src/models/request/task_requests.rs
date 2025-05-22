use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub task_name: String,
    pub description: Option<String>,
    pub editable: Option<bool>,
    pub password: Option<String>,
    pub state: Option<String>
}