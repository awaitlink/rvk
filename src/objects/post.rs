use super::*;

/// <https://vk.com/dev/objects/post>
#[derive(Deserialize, Clone, Debug)]
pub struct Post {
    pub id: Integer,

    pub owner_id: Option<Integer>,
    pub to_id: Option<Integer>,

    pub from_id: Integer,
    pub created_by: Option<Integer>,
    pub date: Integer,
    pub text: String,
    pub reply_owner_id: Option<Integer>,
    pub reply_post_id: Option<Integer>,
    pub friends_only: Option<Integer>,
    pub comments: Comments,
    pub likes: Likes,
    pub reposts: Reposts,
    pub views: Views,
    pub post_type: String,
    pub post_source: Option<post_source::PostSource>,
    pub attachments: Option<Vec<attachment::WallAttachment>>,
    pub geo: Option<geo::Geo>,
    pub signer_id: Option<Integer>,
    pub copy_history: Option<Vec<Post>>,
    pub can_pin: Option<Integer>,
    pub can_delete: Option<Integer>,
    pub can_edit: Option<Integer>,
    pub can_open: Option<Boolean>,
    pub can_close: Option<Boolean>,
    pub is_pinned: Option<Integer>,
    pub marked_as_ads: Integer,
    pub is_favorite: Option<Boolean>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Comments {
    pub count: Integer,
    pub can_post: Integer,
    pub groups_can_post: Option<Boolean>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Likes {
    pub count: Integer,
    pub user_likes: Integer,
    pub can_like: Integer,
    pub can_publish: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Reposts {
    pub count: Integer,
    pub user_reposted: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Views {
    pub count: Integer,
}
