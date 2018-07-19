use super::*;

/// <https://vk.com/dev/objects/comment>
#[derive(Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: Integer,
    pub from_id: Integer,
    pub date: Integer,
    pub text: String,
    pub reply_to_user: Option<Integer>,
    pub reply_to_comment: Option<Integer>,
    pub attachments: Option<Vec<attachment::WallAttachment>>,

    // For attachment::MessageAttachment
    pub post_id: Option<Integer>,
    pub owner_id: Option<Integer>,
}
