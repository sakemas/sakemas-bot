use poise::serenity_prelude::Attachment;
use twapi_v2::{upload::response::Response, headers::Headers};

mod init;
mod append;
mod finalize;

const UPLOAD_URL: &str = "https://upload.twitter.com/1.1/media/upload.json";

#[derive(Debug, thiserror::Error)]
pub enum MediaUploadError {
    #[error("Attachment download error")]
    AttachmentError,
    #[error("Media upload init error: {0}")]
    InitError(twapi_v2::error::Error),
    #[error("Media upload append error: {0}")]
    AppendError(twapi_v2::error::Error),
    #[error("Media upload finalize error: {0}")]
    FinalizeError(twapi_v2::error::Error),
}

pub async fn upload_media(
    token: &str,
    attachment: &Attachment,
    additional_owners: Option<String>,
) -> Result<(Response, Headers), MediaUploadError> {
    // INIT
    let (response, _header) = init::init(token, attachment, additional_owners)
        .await
        .map_err(MediaUploadError::InitError)?;
    let media_id = response.media_id;
    tracing::info!(media_id = media_id, "post_media_upload_init");

    // APPEND
    execute_append(token, attachment, media_id).await?;

    // FINALIZE
    let res = finalize::finalize(token, media_id)
        .await
        .map_err(MediaUploadError::FinalizeError)?;
    tracing::info!(media_id = media_id, "post_media_upload_finalize");
    Ok(res)
}

async fn execute_append(
    token: &str,
    attachment: &Attachment,
    media_id: u64,
) -> Result<(), MediaUploadError> {
    let data = attachment.download().await.map_err(|_| MediaUploadError::AttachmentError)?;
    let mut itr = data.chunks(5000000);

    for (i, chunk) in itr.by_ref().enumerate() {
        append::append(token, media_id, i, chunk)
            .await
            .map_err(MediaUploadError::AppendError)?;

        tracing::info!(
            segment_index = i,
            media_id = media_id,
            "post_media_upload_append"
        );
    }
    Ok(())
}
