use reqwest::multipart::Form;
use twapi_v2::upload::response::Response;

pub async fn finalize(token: &str, media_id: u64) -> Result<Response, twapi_v2::error::Error> {

    let url = "https://upload.twitter.com/1.1/media/upload.json";

    let form = Form::new()
        .text("command", "FINALIZE")
        .text("media_id", media_id.to_string());

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(response)
}
