use line_bot_sdk::models::{
    message::text::TextMessage, message::MessageObject, webhook_event::Location,
};

use crate::error::AppError;

pub fn handler(message: &Location) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!("受け取った住所: {}", message.address))
        .build()
        .into()]))
}
