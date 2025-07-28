use anyhow::Context as _;
use anyhow::Error;
use shuttle_runtime::SecretStore;

/// Get a secret from `Secrets.toml` (or `Secrets.dev.toml` in development)
pub fn get_secret(secret_store: &SecretStore, name: &str) -> Result<String, Error> {
    secret_store
        .get(name)
        .context(format!("'{name}' was not found"))
}

/// Set an environment variable from a secret
pub fn set_env_var(secret_store: &SecretStore, name: &str) -> Result<(), Error> {
    let value = get_secret(secret_store, name)?;
    unsafe {
        std::env::set_var(name, value);
    }
    Ok(())
}
