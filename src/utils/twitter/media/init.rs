use twapi_v2::upload::{response::Response, media_category::MediaCategory};
use poise::serenity_prelude::Attachment;
use reqwest::multipart::Form;

pub async fn init(
    token: &str,
    attachment: &Attachment,
    additional_owners: Option<String>
) -> Result<Response, twapi_v2::error::Error> {
    let media_type = attachment
        .content_type
        .as_ref()
        .expect("attachment content type is missing");
    let total_bytes = attachment.size as u64;
    let media_category = detect_media_category(media_type);

    let url = "https://upload.twitter.com/1.1/media/upload.json";

    let form = Form::new()
        .text("command", "INIT")
        .text("total_bytes", total_bytes.to_string())
        .text("media_type", media_type.to_owned());

    let form = if let Some(media_category) = media_category {
        form.text("media_category", media_category.to_string())
    } else {
        form
    };

    let form = if let Some(additional_owners) = additional_owners {
        form.text("additional_owners", additional_owners)
    } else {
        form
    };

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

fn detect_media_category(content_type: &str) -> Option<MediaCategory> {
    match content_type {
        "image/gif" => Some(MediaCategory::TweetGif),
        t if t.starts_with("image") => Some(MediaCategory::TweetImage),
        t if t.starts_with("video") => Some(MediaCategory::TweetVideo),
        _ => None,
    }
}
