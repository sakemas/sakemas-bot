use std::io::Cursor;
use poise::serenity_prelude::Attachment;
use twapi_v2::{api::{Authentication, BearerAuthentication}, upload::{post_media_upload_append, post_media_upload_init, post_media_upload_finalize, media_category::MediaCategory, response::Response}, headers::Headers};

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
    media_category: Option<MediaCategory>,
    additional_owners: Option<String>,
) -> Result<(Response, Headers), MediaUploadError> {
    // INIT
    let media_type = attachment
        .content_type
        .as_ref()
        .expect("attachment content type is missing");
    let bytes = attachment.download().await.map_err(|_| MediaUploadError::AttachmentError)?;
    let authentication = BearerAuthentication::new(token.to_owned());

    let data = post_media_upload_init::Data {
        total_bytes: attachment.size as u64,
        media_type: media_type.to_owned(),
        media_category,
        additional_owners,
    };
    let (response, _) = post_media_upload_init::Api::new(data)
        .execute(&authentication)
        .await.map_err(MediaUploadError::InitError)?;
    let media_id = response.media_id_string;
    tracing::info!(media_id = media_id, "post_media_upload_init");

    // APPEND
    execute_append(&bytes, &authentication, &media_id).await.map_err(MediaUploadError::AppendError)?;

    // FINALIZE
    let data = post_media_upload_finalize::Data {
        media_id: media_id.clone(),
    };
    let res = post_media_upload_finalize::Api::new(data)
        .execute(&authentication)
        .await.map_err(MediaUploadError::FinalizeError)?;
    tracing::info!(media_id = media_id, "post_media_upload_finalize");
    Ok(res)
}

async fn execute_append(
    data: &[u8],
    authentication: &impl Authentication,
    media_id: &str,
) -> Result<(), twapi_v2::error::Error> {
    let mut itr = data.chunks(5000000);

    for (i, chunk) in itr.by_ref().enumerate() {
        let cursor = Cursor::new(chunk.to_vec());
        let data = post_media_upload_append::Data {
            media_id: media_id.to_owned(),
            segment_index: i as u64,
            cursor,
        };
        let _ = post_media_upload_append::Api::new(data)
            .execute(authentication)
            .await?;

        tracing::info!(
            segment_index = i,
            media_id = media_id,
            "post_media_upload_append"
        );
    }
    Ok(())
}
