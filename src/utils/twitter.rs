use chrono::Utc;
use poise::serenity_prelude::Attachment;
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use sqlx::FromRow;
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

use crate::{utils::secret::get_secret, Data};

pub mod media;

// Refresh Token を用いて Access Token を取得した際のレスポンス(json)からデータを取得するための構造体
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AccessToken {
    // 成功した場合のレスポンス
    pub token_type: Option<String>,
    pub expires_in: Option<i32>,
    pub access_token: Option<String>,
    pub scope: Option<String>,
    pub refresh_token: Option<String>,

    // 失敗した場合のレスポンス
    pub error: Option<String>,
    pub error_description: Option<String>,
}

impl Clone for AccessToken {
    fn clone(&self) -> Self {
        AccessToken {
            token_type: self.token_type.clone(),
            expires_in: self.expires_in,
            access_token: self.access_token.clone(),
            scope: self.scope.clone(),
            refresh_token: self.refresh_token.clone(),
            error: self.error.clone(),
            error_description: self.error_description.clone(),
        }
    }
}

// ツイート際のレスポンス(json)からデータを取得するための構造体
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Tweet {
    pub text: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Media {
    pub media_ids: Option<Vec<u64>>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TweetResult {
    // 成功した場合のレスポンス
    pub data: Option<TweetResultData>,

    // 失敗した場合のレスポンス
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub status: Option<i32>,
    pub detail: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TweetResultData {
    pub edit_history_tweet_ids: Option<Vec<String>>,
    pub id: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, FromRow)]
struct TwitterTokenRow {
    token_type: Option<String>,
    expires_in: Option<i32>,
    access_token: String,
    scope: Option<String>,
    refresh_token: String,
}

#[derive(Debug, Error)]
pub enum TwitterError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Failed to parse data")]
    ParseError,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unhandled error: {0}")]
    Other(String),
}

// curl -X POST https://api.twitter.com/2/oauth2/token \
// --basic -u "<あなたの Client ID>:<あなたの Client Secret>" \
// -H "Content-Type: application/x-www-form-urlencoded" \
// -d "grant_type=refresh_token" \
// -d "client_id=<あなたの Client ID>" \
// -d "refresh_token=<あなたの Refresh Token>"
// Twitter からのレスポンスの形式は pub struct AccessToken の実装を参照
pub async fn refresh_access_token(
    token: &mut AccessToken,
    secret_store: &SecretStore,
) -> Result<AccessToken, TwitterError> {
    let client_id = match get_secret(secret_store, "TWITTER_CLIENT_ID") {
        Ok(v) => v,
        Err(e) => return Err(TwitterError::Other(e.to_string())),
    };
    let client_secret = match get_secret(secret_store, "TWITTER_CLIENT_SECRET") {
        Ok(v) => v,
        Err(e) => return Err(TwitterError::Other(e.to_string())),
    };
    let mut params = HashMap::new();
    let client = reqwest::Client::new();

    params.insert("grant_type", "refresh_token");
    params.insert("client_id", &client_id);
    params.insert("refresh_token", token.refresh_token.as_ref().unwrap());

    let result = client
        .post("https://api.twitter.com/2/oauth2/token")
        .basic_auth(&client_id, Some(&client_secret)) // general_purpose::STANDARD.encode(&format!("{}:{}", client_id, client_secret)); と同じ処理
        // .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded") は自動で設定されるので不要
        .form(&params)
        .send()
        .await
        .map_err(TwitterError::from)?
        .json::<AccessToken>()
        .await;

    println!("{:#?}", result);

    if let Ok(ref t) = result {
        *token = t.clone();
    }

    match result {
        Ok(v) => Ok(v),
        Err(_) => Err(TwitterError::ParseError),
    }
}

// curl -X POST https://api.twitter.com/2/tweets \
// -H "Authorization: Bearer <あなたの Access Token>" \
// -H "Content-Type: application/json; charset=utf-8" \
// -d '{"text":"ツイートテスト"}'
// Twitter からのレスポンスの形式は pub struct Tweet の実装を参照
pub async fn tweet(
    token: &AccessToken,
    text: &str,
    attachments: &Vec<Attachment>,
) -> Result<TweetResult, Box<dyn std::error::Error + Send + Sync>> {
    if attachments.len() > 4 {
        return Err(Box::new(TwitterError::Other(
            "The number of attachments must be less than or equal to 4.".to_string(),
        )));
    }

    /*let media = match attachments.len() {
        0 => None,
        _ => Some(Media {
            media_ids: Some(upload_media(token.access_token.as_ref().unwrap(), attachments).await?),
        }),
    };*/

    let tweet = Tweet {
        text: Some(text.to_string()),
    };
    let payload = serde_json::to_string(&tweet)?;
    let client = reqwest::Client::new();
    let result = client
        .post("https://api.twitter.com/2/tweets")
        .bearer_auth(token.access_token.as_ref().unwrap())
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send()
        .await?
        .json::<TweetResult>()
        .await;

    println!("{:#?}", result);

    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(TwitterError::Other(e.to_string()))),
    }
}

async fn upload_media(
    token: &str,
    attachments: &Vec<Attachment>,
) -> Result<Vec<u64>, Box<dyn std::error::Error + Send + Sync>> {
    let mut media_ids = Vec::new();

    let mut tasks = Vec::new();

    for attachment in attachments {
        let token = token.to_string();
        let attachment = attachment.clone();

        let task = tokio::spawn(async move { media::upload_media(&token, &attachment).await });

        tasks.push(task);
    }

    for task in tasks {
        let media_id = task.await??.media_id;
        media_ids.push(media_id);
    }

    Ok(media_ids)
}

pub async fn get_access_token(
    data: &Data,
) -> Result<AccessToken, Box<dyn std::error::Error + Send + Sync>> {
    let pool = &data.pool;
    let secret_store = &data.secret_store;

    let row = sqlx::query_as::<_, TwitterTokenRow>(
        "SELECT token_type, expires_in, access_token, scope, refresh_token FROM twitter_tokens",
    )
    .fetch_one(pool)
    .await?;

    let mut token = AccessToken {
        token_type: row.token_type,
        expires_in: row.expires_in,
        access_token: Some(row.access_token),
        scope: row.scope,
        refresh_token: Some(row.refresh_token),
        error: None,
        error_description: None,
    };

    {
        let now = Utc::now();
        let mut twitter_token_refreshed_at = data.twitter_token_refreshed_at.lock().unwrap();

        if let Some(refreshed_at) = twitter_token_refreshed_at.as_ref() {
            // if token is not expired, return it
            let duration = now.signed_duration_since(*refreshed_at);
            let duration = duration.to_std()?;
            let expires_in = token.expires_in.unwrap_or(7200);
            if duration < Duration::from_secs(expires_in as u64) {
                return Ok(token);
            }
        }

        let _ = twitter_token_refreshed_at.replace(now);
    }

    let result = refresh_access_token(&mut token, secret_store).await?;

    sqlx::query(
        "UPDATE twitter_tokens
        SET token_type = $1,
            expires_in = $2,
            access_token = $3,
            scope = $4,
            refresh_token = $5
        WHERE id = 1",
    )
    .bind(result.token_type.clone())
    .bind(result.expires_in)
    .bind(result.access_token.clone().unwrap())
    .bind(result.scope.clone())
    .bind(result.refresh_token.clone().unwrap())
    .execute(pool)
    .await?;

    Ok(result)
}
