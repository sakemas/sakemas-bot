use reqwest::Client;
use serde::Deserialize;
use tokio::time;

use super::{super::TwitterError, ImageResultData, MediaUploadResult, VideoResultData};

#[derive(Debug, Deserialize)]
pub struct MediaUploadFinalizeResponse {
    pub media_id: u64,
    pub media_id_string: String,
    pub size: u32,
    pub expires_after_secs: u32,
    pub image: Option<ImageResultData>,
    pub video: Option<VideoResultData>,
    pub processing_info: Option<ProcessingInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessingInfo {
    pub state: String,
    pub check_after_secs: u32,
    pub progress_percent: Option<u32>,
    pub error: Option<ProcessingInfoError>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessingInfoError {
    pub code: u32,
    pub name: String,
    pub message: String,
}

pub async fn finalize(
    client: &Client,
    url: &str,
    token: &str,
    media_id: &u64,
) -> Result<MediaUploadResult, Box<dyn std::error::Error + Send + Sync>> {
    let result = client
        .post(url)
        .bearer_auth(token)
        .form(&[("command", "FINALIZE"), ("media_id", &media_id.to_string())])
        .send()
        .await?
        .json::<MediaUploadFinalizeResponse>()
        .await?;

    if result.processing_info.is_some() {
        let check_after_secs = result.processing_info.as_ref().unwrap().check_after_secs;
        status(client, url, token, media_id, check_after_secs).await?;
    }

    let result = MediaUploadResult {
        media_id: result.media_id,
        media_id_string: result.media_id_string,
        size: result.size,
        expires_after_secs: result.expires_after_secs,
        image: result.image,
        video: result.video,
    };

    Ok(result)
}

async fn status(
    client: &Client,
    url: &str,
    token: &str,
    media_id: &u64,
    check_after_secs: u32,
) -> Result<(), TwitterError> {
    time::sleep(time::Duration::from_secs(check_after_secs as u64)).await;

    let response = match client
        .post(url)
        .bearer_auth(token)
        .form(&[("command", "STATUS"), ("media_id", &media_id.to_string())])
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            return Err(TwitterError::from(e));
        }
    };

    let response = response
        .json::<MediaUploadFinalizeResponse>()
        .await
        .map_err(TwitterError::from)?;

    if let Some(info) = response.processing_info {
        match info.state.as_str() {
            "pending" | "in_progress" => {
                Box::pin(status(client, url, token, media_id, info.check_after_secs)).await?;
            }
            "failed" => {
                return Err(TwitterError::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Processing failed: code: {}, name: {}, message: {}",
                        info.error.as_ref().unwrap().code,
                        info.error.as_ref().unwrap().name,
                        info.error.as_ref().unwrap().message
                    ),
                )));
            }
            "succeeded" => {}
            _ => {
                return Err(TwitterError::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unknown processing state: {}", info.state),
                )));
            }
        }
    }

    Ok(())
}
