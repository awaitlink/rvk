use super::*;

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
}

#[derive(Deserialize, Clone, Debug)]
pub struct Size {
    pub url: String,
    pub width: Integer,
    pub height: Integer,

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