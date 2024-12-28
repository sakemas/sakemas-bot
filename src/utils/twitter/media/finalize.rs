use reqwest::multipart::Form;
use twapi_v2::{upload::response::Response, api::execute_twitter, headers::Headers};

pub async fn finalize(token: &str, media_id: u64) -> Result<(Response, Headers), twapi_v2::error::Error> {
    let form = Form::new()
        .text("command", "FINALIZE")
        .text("media_id", media_id.to_string());

    let client = reqwest::Client::new();
    let builder = client
        .post(super::UPLOAD_URL)
        .bearer_auth(token)
        .multipart(form);

    execute_twitter(builder).await
}
