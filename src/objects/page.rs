use super::*;

/// <https://vk.com/dev/objects/page>
#[derive(Deserialize, Clone, Debug)]
pub struct Page {
    pub id: Integer,
    pub group_id: Integer,
    pub creator_id: Integer,
    pub title: String,
    pub current_user_can_edit: Integer,
    pub current_user_can_edit_access: Integer,
    pub who_can_view: Integer,
    pub who_can_edit: Integer,
    pub edited: Integer,
    pub created: Integer,
    pub editor_id: Integer,
    pub views: Integer,
    pub parent: Option<Integer>,
    pub parent2: Option<Integer>,
    pub source: Option<Integer>,
    pub html: Option<Integer>,
    pub view_url: String,
}
