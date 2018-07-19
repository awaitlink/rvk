use super::*;

/// <https://vk.com/dev/objects/link>
#[derive(Deserialize, Clone, Debug)]
pub struct Link {
    pub url: String,
    pub title: String,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub photo: Option<photo::Photo>,
    pub product: Option<Product>,
    pub button: button::Button,
    pub preview_page: String,
    pub preview_url: String,
}

/// <https://vk.com/dev/link_product>
#[derive(Deserialize, Clone, Debug)]
pub struct Product {
    pub price: Price,
}

/// <https://vk.com/dev/price>
#[derive(Deserialize, Clone, Debug)]
pub struct Price {
    pub amount: Integer,
    pub currency: Currency,
    pub text: String,
}

/// <https://vk.com/dev/price>
#[derive(Deserialize, Clone, Debug)]
pub struct Currency {
    pub id: Integer,
    pub name: String,
}
