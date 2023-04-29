use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiKeyError {
    #[error("This API Key seems to have errors! Try to generate another one or check its scopes")]
    Invalid,
    #[error("No DEVTO_API_KEY env var was set! Without it is impossible to pusblish remotely")]
    Missing,
}
