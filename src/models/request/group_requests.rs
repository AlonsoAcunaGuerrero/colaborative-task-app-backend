use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub group_name: String,
    pub description: Option<String>
}