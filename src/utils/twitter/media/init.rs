use poise::serenity_prelude as serenity;
use reqwest::Client;
use serde::Deserialize;
use serenity::Attachment;

use super::{ImageResultData, VideoResultData};

#[derive(Debug, Deserialize)]
pub struct MediaUploadInitResponse {
    pub media_id: u64,
    pub media_id_string: String,
    pub size: u32,
    pub expires_after_secs: u32,
    pub image: Option<ImageResultData>,
    pub video: Option<VideoResultData>,
}

pub async fn init(
    client: &Client,
    url: &str,
    token: &str,
    attachment: &Attachment,
) -> Result<MediaUploadInitResponse, Box<dyn std::error::Error + Send + Sync>> {
    let media_type = attachment
        .content_type
        .as_ref()
        .expect("attachment content type is missing");

    let result = client
        .post(url)
        .bearer_auth(token)
        .form(&[
            ("command", "INIT"),
            ("total_bytes", &attachment.size.to_string()),
            ("media_type", media_type),
        ])
        .send()
        .await?
        .json::<MediaUploadInitResponse>()
        .await;

    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e)),
    }
}
