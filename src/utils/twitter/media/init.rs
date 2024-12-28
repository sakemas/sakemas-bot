use twapi_v2::{upload::{response::Response, media_category::MediaCategory}, api::execute_twitter, headers::Headers};
use poise::serenity_prelude::Attachment;
use reqwest::multipart::Form;

pub async fn init(
    token: &str,
    attachment: &Attachment,
    additional_owners: Option<String>
) -> Result<(Response, Headers), twapi_v2::error::Error> {
    let media_type = attachment
        .content_type
        .as_ref()
        .expect("attachment content type is missing");
    let total_bytes = attachment.size as u64;
    let media_category = detect_media_category(media_type);

    let mut form = Form::new()
        .text("command", "INIT")
        .text("total_bytes", total_bytes.to_string())
        .text("media_type", media_type.to_owned());

    if let Some(media_category) = media_category {
        form = form.text("media_category", media_category.to_string());
    }

    if let Some(additional_owners) = additional_owners {
        form = form.text("additional_owners", additional_owners);
    }

    let client = reqwest::Client::new();
    let builder = client
        .post(super::UPLOAD_URL)
        .bearer_auth(token)
        .multipart(form);

    execute_twitter(builder).await
}

fn detect_media_category(content_type: &str) -> Option<MediaCategory> {
    match content_type {
        "image/gif" => Some(MediaCategory::TweetGif),
        t if t.starts_with("image") => Some(MediaCategory::TweetImage),
        t if t.starts_with("video") => Some(MediaCategory::TweetVideo),
        _ => None,
    }
}
