use chrono::Utc;
use std::time::Duration;
use twapi_v2::{api::post_2_oauth2_token_refresh_token, headers::Headers};

use super::{TwitterError, TwitterTokenRow};
use crate::Data;

pub async fn refresh_access_token(
    token: &mut post_2_oauth2_token_refresh_token::Response,
    data: &Data,
) -> Result<(post_2_oauth2_token_refresh_token::Response, Headers), TwitterError> {
    let client_id = &data.twitter_client_id;
    let client_secret = &data.twitter_client_secret;

    let api = post_2_oauth2_token_refresh_token::Api::new(
        client_id,
        client_secret,
        token.refresh_token.as_ref().unwrap(),
    );

    api.execute().await.map_err(TwitterError::ApiError)
}

pub async fn get_access_token(
    data: &Data,
) -> Result<post_2_oauth2_token_refresh_token::Response, Box<dyn std::error::Error + Send + Sync>> {
    let pool = &data.pool;

    let row = sqlx::query_as::<_, TwitterTokenRow>(
        "SELECT token_type, expires_in, access_token, scope, refresh_token FROM twitter_tokens",
    )
    .fetch_one(pool)
    .await?;

    let mut token = post_2_oauth2_token_refresh_token::Response {
        token_type: row.token_type,
        expires_in: row.expires_in,
        access_token: Some(row.access_token),
        scope: row.scope,
        refresh_token: Some(row.refresh_token),
        extra: Default::default(),
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

    let response = refresh_access_token(&mut token, data).await?.0;

    sqlx::query(
        "UPDATE twitter_tokens
        SET token_type = $1,
            expires_in = $2,
            access_token = $3,
            scope = $4,
            refresh_token = $5
        WHERE id = 1",
    )
    .bind(response.token_type.clone())
    .bind(response.expires_in)
    .bind(response.access_token.clone().unwrap())
    .bind(response.scope.clone())
    .bind(response.refresh_token.clone().unwrap())
    .execute(pool)
    .await?;

    Ok(response)
}
