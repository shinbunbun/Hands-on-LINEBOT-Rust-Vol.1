#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    LineBotSdkError(line_bot_sdk::Error),
    ReqwestError(reqwest::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(errors) => write!(f, "Bad Request: {}", errors),
            AppError::LineBotSdkError(errors) => write!(f, "line bot sdk error: {}", errors),
            AppError::ReqwestError(errors) => write!(f, "reqwest error: {}", errors),
        }
    }
}
