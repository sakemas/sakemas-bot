use poise::serenity_prelude::Attachment;
use shuttle_runtime::SecretStore;
use sqlx::FromRow;
use thiserror::Error;
use twapi_v2::{
    api::{
        delete_2_tweets_id, get_2_tweets_id, post_2_oauth2_token_refresh_token, post_2_tweets::{self, Media}, BearerAuthentication
    },
    error::Error,
};

use crate::utils::secret::get_secret;

mod access_token;
pub mod media;

pub use access_token::{get_access_token, refresh_access_token};

#[derive(Debug, FromRow)]
struct TwitterTokenRow {
    token_type: Option<String>,
    expires_in: Option<i64>,
    access_token: String,
    scope: Option<String>,
    refresh_token: String,
}

#[derive(Debug, Error)]
pub enum TwitterError {
    #[error("API error: {0}")]
    ApiError(Error),
    #[error("Not found the post")]
    NotFound,
    #[error("Unhandled error: {0}")]
    Other(String),
}

pub async fn tweet(
    token: &post_2_oauth2_token_refresh_token::Response,
    text: &str,
    attachments: &[Attachment],
) -> Result<post_2_tweets::Response, Box<dyn std::error::Error + Send + Sync>> {
    if attachments.len() > 4 {
        return Err(Box::new(TwitterError::Other(
            "The number of attachments must be less than or equal to 4.".to_string(),
        )));
    }

    let access_token = token.access_token.as_ref().unwrap();

    let auth = BearerAuthentication::new(access_token);

    let media = match attachments.len() {
        0 => None,
        _ => Some(Media {
            media_ids: upload_media(access_token, attachments).await?,
            tagged_user_ids: Vec::new(),
        }),
    };

    let body = post_2_tweets::Body {
        text: Some(text.to_string()),
        media,
        ..Default::default()
    };
    let (response, _header) = post_2_tweets::Api::new(body).execute(&auth).await.map_err(|e| {
        Box::new(TwitterError::Other(e.to_string()))
    })?;

    Ok(response)
}

pub async fn upload_media(
    token: &str,
    attachments: &[Attachment],
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut media_ids = Vec::new();

    let mut tasks = Vec::new();

    for attachment in attachments {
        let token = token.to_owned();
        let attachment = attachment.clone();

        let task = tokio::spawn(async move { media::upload_media(&token, &attachment, vec![]).await });

        tasks.push(task);
    }

    for task in tasks {
        let media_id = task.await??;
        media_ids.push(media_id.to_string());
    }

    Ok(media_ids)
}

pub async fn check_is_our_post(
    token: &str,
    post_id: &str,
    secret_store: &SecretStore,
) -> Result<Option<bool>, TwitterError> {
    let auth = BearerAuthentication::new(token);

    let (response, _header) = get_2_tweets_id::Api::new(post_id).execute(&auth).await.map_err(|e| {
        TwitterError::ApiError(e)
    })?;
    let tweet = match response.data {
        Some(data) => data,
        None => return Err(TwitterError::NotFound),
    };
    let author = tweet.author_id.as_ref();

    Ok(
        match author {
            Some(author) => Some(author == &get_secret(secret_store, "TWITTER_CLIENT_ID").map_err(|e| {
                TwitterError::Other(e.to_string())
            })?),
            None => None,
        }
    )
}

pub async fn delete_post(
    token: &str,
    post_id: &str,
) -> Result<(), TwitterError> {
    let auth = BearerAuthentication::new(token);

    let (_response, _header) = delete_2_tweets_id::Api::new(post_id).execute(&auth).await.map_err(|e| {
        TwitterError::ApiError(e)
    })?;

    Ok(())
}
