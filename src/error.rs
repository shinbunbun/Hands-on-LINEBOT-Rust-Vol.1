use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("line bot sdk error: {0}")]
    LineBotSdk(#[from] line_bot_sdk::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("env error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("internal error: {0}")]
    Internal(String),
}
