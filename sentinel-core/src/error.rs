use thiserror::Error;

#[derive(Error, Debug)]
pub enum SentinelError {
    #[error("HTTP request error")]
    Http(String),
    #[error("JSON parsing error")]
    Parse(#[from] serde_json::Error),
    #[error("Url {url} is unreachable")]
    Unreachable { url: String },
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Agent error: {0}")]
    Agent(String),
}
