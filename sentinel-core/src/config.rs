use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelConfig {
    pub model_url: String,
    pub model: String,
    pub max_concurrency: usize,
    pub request_timeout_secs: u64,
}

impl Default for SentinelConfig {
    fn default() -> Self {
        Self {
            model_url: String::from("http://localhost:11434"),
            model: String::from("qwen3:30b"),
            max_concurrency: 20,
            request_timeout_secs: 10,
        }
    }
}
