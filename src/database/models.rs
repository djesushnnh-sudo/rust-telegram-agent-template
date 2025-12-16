use serde::{Deserialize, Serialize};
use teloxide::types::{ChatId, MessageId};
use chrono::{DateTime, Utc};

/// User model representing a Telegram bot user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub telegram_id: i64,
    pub username: String,
    pub created_at: String,
}

impl User {
    /// Create a new user instance
    pub fn new(telegram_id: i64, username: String) -> Self {
        Self {
            id: 0, // Will be set by database
            telegram_id,
            username,
            created_at: String::new(), // Will be set by database
        }
    }
}

/// Managed group model (based on Dads2Dads patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedGroup {
    pub chat_id: ChatId,
    pub group_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Forwarded message mapping (based on Dads2Dads patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardedMessage {
    pub admin_msg_id: MessageId,
    pub original_chat_id: ChatId,
    pub original_msg_id: MessageId,
    pub created_at: DateTime<Utc>,
}

/// User preferences for AI and other features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub user_id: i64,
    pub ai_enabled: bool,
    pub language: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}