use serde::{Deserialize, Serialize};

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