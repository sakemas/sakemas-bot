use std::io::Cursor;
use twapi_v2::{
    api::{
        post_2_media_upload_id_append, post_2_media_upload_id_finalize,
        post_2_media_upload_initialize::{self, MediaCategory},
        Authentication, TwapiOptions,
    },
    error::Error,
    headers::Headers,
};

pub async fn upload_media_from_bytes(
    data: &[u8],
    media_type: &str,
    media_category: Option<MediaCategory>,
    additional_owners: Vec<String>,
    authentication: &impl Authentication,
    twapi_options: Option<&TwapiOptions>,
) -> Result<(post_2_media_upload_id_finalize::Response, Headers), Error> {
    // INIT
    let file_size = data.len() as u64;
    let media_id = execute_init(
        file_size,
        media_type,
        media_category,
        additional_owners,
        authentication,
        twapi_options,
    )
    .await?;
    tracing::info!(media_id = media_id, "post_media_upload_init");

    // APPEND
    execute_append_from_bytes(data, authentication, file_size, &media_id, twapi_options).await?;

    // FINALIZE
    let mut api = post_2_media_upload_id_finalize::Api::new(&media_id);
    if let Some(twapi_options) = twapi_options {
        api = api.twapi_options(twapi_options.clone());
    }
    let res = api.execute(authentication).await;
    tracing::info!(media_id = media_id, "post_media_upload_finalize");
    res
}

async fn execute_init(
    file_size: u64,
    media_type: &str,
    media_category: Option<MediaCategory>,
    additional_owners: Vec<String>,
    authentication: &impl Authentication,
    twapi_options: Option<&TwapiOptions>,
) -> Result<String, Error> {
    let body = post_2_media_upload_initialize::Body {
        total_bytes: file_size,
        media_type: media_type.to_owned(),
        media_category,
        additional_owners,
    };
    let mut api = post_2_media_upload_initialize::Api::new(body);
    if let Some(twapi_options) = twapi_options {
        api = api.twapi_options(twapi_options.clone());
    }
    let (response, _) = api.execute(authentication).await?;
    let media_id = response.data.and_then(|it| it.id).unwrap_or_default();
    Ok(media_id)
}

async fn execute_append_from_bytes(
    data: &[u8],
    authentication: &impl Authentication,
    file_size: u64,
    media_id: &str,
    twapi_options: Option<&TwapiOptions>,
) -> Result<(), Error> {
    let mut segment_index = 0;
    while segment_index * 5000000 < file_size {
        let start_pos = segment_index as usize * 5000000;
        let remaining_bytes = file_size as usize - start_pos;
        let read_size = std::cmp::min(5000000, remaining_bytes);

        let data_slice = &data[start_pos..start_pos + read_size];
        let cursor = Cursor::new(data_slice.to_owned());

        let form = post_2_media_upload_id_append::FormData {
            segment_index,
            cursor,
        };
        let mut api = post_2_media_upload_id_append::Api::new(media_id, form);
        if let Some(twapi_options) = twapi_options {
            api = api.twapi_options(twapi_options.clone());
        }
        let _ = api.execute(authentication).await?;
        tracing::info!(
            segment_index = segment_index,
            media_id = media_id,
            "post_media_upload_append"
        );
        segment_index += 1;
    }
    Ok(())
}
