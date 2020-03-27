use super::*;

/// <https://vk.com/dev/objects/photo>
#[derive(Deserialize, Clone, Debug)]
pub struct Photo {
    pub id: Integer,
    pub album_id: Integer,
    pub owner_id: Integer,
    pub user_id: Option<Integer>,
    pub text: String,
    pub date: Integer,
    pub sizes: Vec<Size>,
    pub width: Option<Integer>,
    pub height: Option<Integer>,

    // for attachment::WallAttachment
    pub post_id: Option<Integer>,

    /// Access key may be present in attachments
    /// (
    /// <https://vk.com/dev/objects/attachments_w>
    /// or
    /// <https://vk.com/dev/objects/attachments_m>
    /// )
    pub access_key: Option<String>,
}

/// <https://vk.com/dev/photo_sizes>
#[derive(Deserialize, Clone, Debug)]
pub struct Size {
    pub url: String,
    pub width: Option<Integer>,
    pub height: Option<Integer>,

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Cropped {
    pub photo: Photo,
    pub crop: Rect,
    pub rect: Rect,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Rect {
    pub x: Number,
    pub y: Number,
    pub x2: Number,
    pub y2: Number,
}

/// <https://vk.com/dev/objects/attachments_w>
#[derive(Deserialize, Clone, Debug)]
pub struct Album {
    pub id: Integer,
    pub thumb: photo::Photo,
    pub owner_id: Integer,
    pub title: String,
    pub description: String,
    pub created: Integer,
    pub updated: Integer,
    pub size: Integer,
}

/// <https://vk.com/dev/objects/attachments_w>
/// or
/// <https://vk.com/dev/objects/sticker>
#[derive(Deserialize, Clone, Debug)]
pub struct Image {
    pub url: String,
    pub width: Integer,
    pub height: Integer,
}
