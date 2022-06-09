use super::*;

/// <https://vk.com/dev/objects/video>
#[derive(Deserialize, Clone, Debug)]
pub struct Video {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub description: String,
    pub duration: Integer,

    pub image: Option<Vec<VideoImage>>,
    // from changelog v5.101 @ https://vk.com/dev/versions
    pub photo_130: Option<String>,
    pub photo_320: Option<String>,
    pub photo_640: Option<String>,
    pub photo_800: Option<String>,
    pub photo_1280: Option<String>,

    pub first_frame: Option<Vec<VideoImage>>,
    // from changelog v5.101 @ https://vk.com/dev/versions
    pub first_frame_130: Option<String>,
    pub first_frame_160: Option<String>,
    // can be present inside https://vk.com/dev/objects/story
    pub first_frame_320: Option<String>,
    pub first_frame_640: Option<String>,
    pub first_frame_800: Option<String>,
    pub first_frame_1280: Option<String>,

    pub date: Integer,
    pub adding_date: Option<Integer>,
    pub views: Integer,
    pub comments: Integer,
    pub player: Option<String>,
    pub platform: Option<String>,
    pub can_edit: Option<Integer>,
    pub can_add: Integer,
    pub is_private: Option<Integer>,
    pub access_key: Option<String>,
    pub processing: Option<Integer>,
    pub live: Option<Integer>,
    pub upcoming: Option<Integer>,
    pub is_favorite: Option<Boolean>,
}

/// <https://vk.com/dev/objects/video_image>
#[derive(Deserialize, Clone, Debug)]
pub struct VideoImage {
    pub url: String,
    pub width: Integer,
    pub height: Integer,
    pub with_padding: Option<Integer>,
}

/// <https://vk.com/dev/objects/video_album_full>
#[derive(Deserialize, Clone, Debug)]
pub struct VideoPlaylist {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub count: Integer,
    pub photo160: Option<String>,
    pub photo320: Option<String>,
    pub image: Option<Vec<VideoImage>>,
    pub updated_time: Option<Integer>,
    pub is_system: Option<Integer>,
    pub privacy: Option<super::privacy::Privacy>,
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
