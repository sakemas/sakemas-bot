CREATE TABLE IF NOT EXISTS twitter_tokens (
    id INT PRIMARY KEY NOT NULL,
    token_type VARCHAR,
    expires_in INT8,
    access_token VARCHAR NOT NULL,
    scope VARCHAR,
    refresh_token VARCHAR NOT NULL
);
