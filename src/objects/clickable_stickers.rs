use super::*;

/// <https://vk.com/dev/objects/clickable_stickers>
#[derive(Deserialize, Clone, Debug)]
pub struct ClickableStickers {
    pub original_width: Integer,
    pub original_height: Integer,
    pub clickable_stickers: Vec<ClickableSticker>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ClickableSticker {
    #[serde(rename = "type")]
    pub type_: String,
    pub clickable_area: Vec<Point2D>,
    pub style: Option<String>,
    pub mention: Option<String>,
    pub hashtag: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Point2D {
    pub x: Integer,
    pub y: Integer,
}
