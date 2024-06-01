use poise::serenity_prelude::Attachment;
use reqwest::Client;
use serde::Deserialize;

mod append;
mod finalize;
mod init;

#[derive(Debug, Deserialize)]
pub struct MediaUploadResult {
    pub media_id: u64,
    pub media_id_string: String,
    pub size: u32,
    pub expires_after_secs: u32,
    pub image: Option<ImageResultData>,
    pub video: Option<VideoResultData>,
}

#[derive(Debug, Deserialize)]
struct ImageUploadResult {
    media_id: u64,
    media_id_string: String,
    size: u32,
    expires_after_secs: u32,
    pub image: ImageResultData,
}

#[derive(Debug, Deserialize)]
struct VideoUploadResult {
    media_id: u64,
    media_id_string: String,
    size: u32,
    expires_after_secs: u32,
    pub video: VideoResultData,
}

#[derive(Debug, Deserialize)]
struct ImageResultData {
    image_type: String,
    w: Option<u32>,
    h: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct VideoResultData {
    video_type: String,
}

pub struct MediaUploadData {
    media_data: Vec<u8>,
    media_type: String,
    media_category: Option<String>,
}

impl MediaUploadData {
    pub fn builder() -> MediaUploadDataBuilder {
        MediaUploadDataBuilder::default()
    }
}

#[derive(Default)]
pub struct MediaUploadDataBuilder {
    media_data: Vec<u8>,
    media_type: String,
    media_category: Option<String>,
}

impl MediaUploadDataBuilder {
    pub fn media_data(&mut self, media_data: Vec<u8>) -> &mut Self {
        self.media_data = media_data;
        self
    }

    pub fn media_category(&mut self, media_category: String) -> &mut Self {
        self.media_category = Some(media_category);
        self
    }

    pub fn media_type(&mut self, media_type: String) -> &mut Self {
        self.media_type = media_type;
        self
    }

    pub fn build(&self) -> MediaUploadData {
        MediaUploadData {
            media_data: self.media_data.clone(),
            media_category: self.media_category.clone(),
            media_type: self.media_type.clone(),
        }
    }
}

pub async fn upload_media(
    token: &str,
    attachment: &Attachment,
) -> Result<MediaUploadResult, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let url = "https://upload.twitter.com/1.1/media/upload.json";

    let data = attachment.download().await?;

    let init_response = init::init(&client, url, token, attachment).await?;
    let media_id = init_response.media_id;

    append::append(&client, url, token, &media_id, data).await?;

    finalize::finalize(&client, url, token, &media_id).await
}
