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

    /// Access key may be present in attachments
    /// (
    /// <https://vk.com/dev/objects/attachments_w>
    /// or
    /// <https://vk.com/dev/objects/attachments_m>
    /// )
    pub access_key: Option<String>,
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

/// <https://vk.com/dev/objects/video_cat_element>
#[derive(Deserialize, Clone, Debug)]
pub struct CatalogElement {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,

    #[serde(rename = "type")]
    pub type_: String,

    pub description: Option<String>,
    pub duration: Option<Integer>,
    pub photo_130: Option<String>,
    pub photo_160: Option<String>,
    pub photo_320: Option<String>,
    pub photo_640: Option<String>,
    pub photo_800: Option<String>,
    pub date: Option<Integer>,
    pub adding_date: Option<Integer>,
    pub views: Option<Integer>,
    pub comments: Option<Integer>,
    pub can_add: Option<Integer>,
    pub can_edit: Option<Integer>,
    pub is_private: Option<Integer>,
    pub count: Option<Integer>,
    pub updated_time: Option<Integer>,
}

/// <https://vk.com/dev/objects/video_cat_block>
#[derive(Deserialize, Clone, Debug)]
pub struct CatalogBlock {
    pub items: Vec<CatalogElement>,
    pub id: Integer,
    pub name: String,
    pub next: String,
    pub view: String,
    pub can_hide: Integer,

    #[serde(rename = "type")]
    pub type_: String,
}
