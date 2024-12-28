use reqwest::multipart::{Form, Part};

pub async fn append(
    token: &str,
    media_id: u64,
    segment_index: usize,
    chunk: &[u8],
) -> Result<(), twapi_v2::error::Error> {

    let url = "https://upload.twitter.com/1.1/media/upload.json";

    let form = Form::new()
        .text("command", "APPEND")
        .text("media_id", media_id.to_string())
        .text("segment_index", segment_index.to_string())
        .part("media", Part::bytes(chunk.to_vec()));

    let client = reqwest::Client::new();
    client
        .post(url)
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?;

    Ok(())
}
