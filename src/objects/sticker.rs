use super::*;

/// <https://vk.com/dev/objects/sticker>
#[derive(Deserialize, Clone, Debug)]
pub struct Sticker {
    pub product_id: Option<Integer>,
    pub sticker_id: Integer,
    pub images: Vec<photo::Image>,
    pub images_with_background: Vec<photo::Image>,
}
