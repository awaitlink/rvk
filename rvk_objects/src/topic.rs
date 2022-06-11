use super::*;

/// <https://vk.com/dev/objects/topic>
#[derive(Deserialize, Clone, Debug)]
pub struct Topic {
    pub id: Integer,
    pub title: String,
    pub created: Integer,
    pub created_by: Integer,
    pub updated: Integer,
    pub updated_by: Integer,
    pub is_closed: Integer,
    pub is_fixed: Integer,
    pub comments: Integer,
    pub first_comment: Option<String>,
    pub last_comment: Option<String>,
}
