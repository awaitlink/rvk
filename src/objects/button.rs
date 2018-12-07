use super::*;

/// <https://vk.com/dev/link_button>
#[derive(Deserialize, Clone, Debug)]
pub struct Button {
    pub title: String,
    pub action: ButtonAction,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ButtonAction {
    #[serde(rename = "type")]
    pub type_: String,

    pub url: String, // Make Option<String> if multiple types will be added
}
