use super::*;

/// <https://vk.com/dev/objects/video>
#[derive(Deserialize, Clone, Debug)]
pub struct Video {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub description: String,
    pub duration: Integer,
    pub date: Integer,
    pub adding_date: Integer,
    pub views: Integer,
    pub comments: Integer,
    pub player: String,
    pub platform: Option<String>,
    pub can_edit: Option<Integer>,
    pub can_add: Integer,
    pub is_private: Option<Integer>,
    pub access_key: String,
    pub processing: Option<Integer>,
    pub converting: Option<Integer>,
    pub live: Option<Integer>,
    pub upcoming: Option<Integer>,

    pub photo_130: Option<String>,
    pub photo_320: Option<String>,
    pub photo_640: Option<String>,
    pub photo_800: Option<String>,

    pub first_frame_130: Option<String>,
    pub first_frame_160: Option<String>,
    pub first_frame_320: Option<String>,
    pub first_frame_800: Option<String>,

    pub files: Option<Files>,

    // Additional (extended) fields
    pub likes: Option<Likes>,
    pub reposts: Option<Reposts>,
    pub can_comment: Option<Integer>,
    pub can_repost: Option<Integer>,
    pub repeat: Option<Integer>,
    pub privacy_view: Option<privacy::Privacy>,
    pub privacy_comment: Option<privacy::Privacy>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Likes {
    pub user_likes: Integer,
    pub count: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Reposts {
    pub user_reposted: Integer,
    pub count: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Files {
    pub mp4_240: Option<String>,
    pub mp4_360: Option<String>,
    pub mp4_480: Option<String>,
    pub mp4_720: Option<String>,
    pub mp4_1080: Option<String>,
}