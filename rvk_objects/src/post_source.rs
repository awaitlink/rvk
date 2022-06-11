use super::*;

/// <https://vk.com/dev/objects/post_source>
#[derive(Deserialize, Clone, Debug)]
pub struct PostSource {
    #[serde(rename = "type")]
    pub type_: String,

    pub platform: Option<String>,
    pub data: Option<String>,
    pub url: Option<String>,
}
