use super::*;

/// <https://vk.com/dev/objects/doc>
#[derive(Deserialize, Clone, Debug)]
pub struct Document {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub size: Integer,
    pub ext: String,
    pub url: String,
    pub date: Integer,

    #[serde(rename = "type")]
    pub type_: Integer,

    pub preview: DocumentPreview,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DocumentPreview {
    pub photo: Option<Photo>,
    pub graffiti: Option<Graffiti>,
    pub audio_msg: Option<AudioMessage>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Photo {
    pub sizes: Vec<photo::Size>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Graffiti {
    pub src: String,
    pub width: Integer,
    pub height: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AudioMessage {
    pub duration: Integer,
    pub waveform: Vec<Integer>,
    pub link_ogg: String,
    pub link_mp3: String,
}
