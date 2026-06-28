use anyhow::{Context as _, Error};

/// Get a secret from the environment.
///
/// In development, values are loaded from `.env` by `dotenvy` before this is called.
/// In production, values must be present in the process environment.
pub fn get_secret(name: &str) -> Result<String, Error> {
    std::env::var(name).with_context(|| format!("'{name}' was not found"))
}
