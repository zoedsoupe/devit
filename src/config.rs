use crate::errors::ApiKeyError;
use anyhow::Result;
use std::env;

const API_KEY_ENV: &str = "DEVTO_API_KEY";

pub fn get_api_key() -> Result<String, ApiKeyError> {
    let api_key = env::var(API_KEY_ENV);

    if api_key.is_err() {
        return Err(ApiKeyError::Missing);
    }

    Ok(api_key.unwrap())
}
