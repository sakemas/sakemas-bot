use std::io::Cursor;
use poise::serenity_prelude::Attachment;
use twapi_v2::{api::{Authentication, BearerAuthentication}, upload::{post_media_upload_append, post_media_upload_init, post_media_upload_finalize, media_category::MediaCategory, response::Response}, headers::Headers};

pub async fn upload_media(
    token: &str,
    attachment: &Attachment,
    media_category: Option<MediaCategory>,
    additional_owners: Option<String>,
) -> anyhow::Result<(Response, Headers)> {
    // INIT
    let media_type = attachment
        .content_type
        .as_ref()
        .expect("attachment content type is missing");
    let bytes = attachment.download().await?;
    let file_size = bytes.len() as u64;
    let authentication = BearerAuthentication::new(token.to_owned());

    let data = post_media_upload_init::Data {
        total_bytes: file_size,
        media_type: media_type.to_owned(),
        media_category,
        additional_owners,
    };
    let (response, _) = post_media_upload_init::Api::new(data)
        .execute(&authentication)
        .await?;
    let media_id = response.media_id_string;
    tracing::info!(media_id = media_id, "post_media_upload_init");

    // APPEND
    execute_append(&bytes, &authentication, file_size, &media_id).await?;

    // FINALIZE
    let data = post_media_upload_finalize::Data {
        media_id: media_id.clone(),
    };
    let res = post_media_upload_finalize::Api::new(data)
        .execute(&authentication)
        .await?;
    tracing::info!(media_id = media_id, "post_media_upload_finalize");
    Ok(res)
}

async fn execute_append(
    data: &[u8],
    authentication: &impl Authentication,
    file_size: u64,
    media_id: &str,
) -> anyhow::Result<()> {
    let mut segment_index = 0;
    while segment_index * 5000000 < file_size {
        let read_size: usize = if (segment_index + 1) * 5000000 < file_size {
            5000000
        } else {
            (file_size - segment_index * 5000000) as usize
        };
        let chunk = data[segment_index as usize * 5000000..(segment_index as usize * 5000000 + read_size)].to_vec();
        let cursor = Cursor::new(chunk);
        let data = post_media_upload_append::Data {
            media_id: media_id.to_owned(),
            segment_index,
            cursor,
        };
        let _ = post_media_upload_append::Api::new(data)
            .execute(authentication)
            .await?;
        tracing::info!(
            segment_index = segment_index,
            media_id = media_id,
            "post_media_upload_append"
        );
        segment_index += 1;
    }
    Ok(())
}
