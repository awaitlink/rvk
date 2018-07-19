use super::*;

/// <https://vk.com/dev/objects/comment_board>
#[derive(Deserialize, Clone, Debug)]
pub struct BoardComment {
    pub id: Integer,
    pub from_id: Integer,
    pub date: Integer,
    pub text: String,
    pub attachments: Option<Vec<attachment::WallAttachment>>,
    pub likes: Option<Likes>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Likes {
    pub count: Integer,
    pub user_likes: Integer,
    pub can_like: Integer,
}
