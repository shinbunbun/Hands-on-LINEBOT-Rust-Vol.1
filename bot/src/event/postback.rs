use line_bot_sdk::{
    error::AppError,
    models::{
        message::{text::TextMessage, MessageObject},
        webhook_event::Event,
    },
};

pub async fn index(event: &Event) -> Result<Option<Vec<MessageObject>>, AppError> {
    let postback = event
        .postback
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("postback not found".to_string()))?;
    let res = match &postback.params {
        Some(params) => vec![MessageObject::Text(TextMessage::new(format!(
            "日時データを受け取りました！\ndata: {}\ndatetime: {}",
            postback.data,
            params
                .datetime
                .clone()
                .ok_or_else(|| AppError::BadRequest("datetime not found".to_string()))?
        )))],
        None => vec![MessageObject::Text(TextMessage::new(format!(
            "ポストバックデータを受け取りました！\ndata: {}",
            postback.data
        )))],
    };
    Ok(Some(res))
}