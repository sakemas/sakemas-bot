use poise::serenity_prelude::Attachment;
use twapi_v2::{
    api::{post_2_media_upload_initialize::MediaCategory, BearerAuthentication},
    upload_v2::{check_processing, get_media_id},
};

mod upload_from_bytes; // temp
use upload_from_bytes::upload_media_from_bytes;

#[derive(Debug, thiserror::Error)]
pub enum MediaUploadError {
    #[error("Attachment download error")]
    AttachmentError,
    #[error("Media upload error: {0}")]
    UploadError(twapi_v2::error::Error),
}

pub async fn upload_media(
    token: &str,
    attachment: &Attachment,
    additional_owners: Vec<String>,
) -> Result<String, MediaUploadError> {
    let auth = BearerAuthentication::new(token);

    let media_type = attachment
        .content_type
        .as_ref()
        .expect("attachment content type is missing");
    let media_category = detect_media_category(media_type);

    let data = attachment.download().await.map_err(|_| MediaUploadError::AttachmentError)?;

    let (response, _header) = upload_media_from_bytes(
        &data,
        media_type,
        media_category,
        additional_owners,
        &auth,
        None,
    )
    .await
    .map_err(MediaUploadError::UploadError)?;

    let media_id = get_media_id(&response);

    tracing::info!(media_id = media_id, "start uploading media");

    check_processing(
        response,
        &auth,
        Some(|count, _response: &_, _header: &_| {
            if count > 100 {
                Err(twapi_v2::error::Error::Upload("over counts".to_owned()))
            } else {
                Ok(())
            }
        }),
        None,
    )
    .await
    .map_err(MediaUploadError::UploadError)?;
    tracing::info!(media_id = media_id, "end uploading media");

    Ok(media_id)
}

fn detect_media_category(content_type: &str) -> Option<MediaCategory> {
    match content_type {
        "image/gif" => Some(MediaCategory::TweetGif),
        t if t.starts_with("image") => Some(MediaCategory::TweetImage),
        t if t.starts_with("video") => Some(MediaCategory::TweetVideo),
        _ => None,
    }
}
