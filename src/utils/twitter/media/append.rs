use reqwest::multipart::{Form, Part};
use twapi_v2::{api::execute_twitter, headers::Headers};

pub async fn append(
    token: &str,
    media_id: u64,
    segment_index: usize,
    chunk: &[u8],
) -> Result<((), Headers), twapi_v2::error::Error> {
    let form = Form::new()
        .text("command", "APPEND")
        .text("media_id", media_id.to_string())
        .text("segment_index", segment_index.to_string())
        .part("media", Part::bytes(chunk.to_vec()));

    let client = reqwest::Client::new();
    let builder = client
        .post(super::UPLOAD_URL)
        .bearer_auth(token)
        .multipart(form);

    execute_twitter(builder).await
}
