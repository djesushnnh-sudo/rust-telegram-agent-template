use async_trait::async_trait;
use anyhow::Result;
use teloxide::types::{ChatId, UserId, MessageId};
use serde::{Deserialize, Serialize};

use crate::database::models::{User, ManagedGroup, ForwardedMessage};

/// Database statistics for monitoring
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub users_count: u64,
    pub groups_count: u64,
    pub forwarded_messages_count: u64,
}

/// Database provider trait for database-agnostic operations
/// 
/// This trait defines the interface for all database operations in the bot.
/// Implementations can use different backends (SQLite, PostgreSQL, in-memory, etc.)
/// while maintaining the same API for the rest of the application.
#[async_trait]
pub trait DatabaseProvider: Send + Sync {
    // =========================================================================
    // USER OPERATIONS
    // =========================================================================
    
    /// Create a new user
    async fn create_user(&self, username: &str, telegram_id: i64) -> Result<()>;
    
    /// Get a user by their Telegram ID
    async fn get_user(&self, telegram_id: i64) -> Result<Option<User>>;
    
    /// Get all users
    async fn get_all_users(&self) -> Result<Vec<User>>;
    
    /// Delete a user by their Telegram ID
    async fn delete_user(&self, telegram_id: i64) -> Result<bool>;

    // =========================================================================
    // MANAGED GROUPS OPERATIONS
    // =========================================================================

    /// Store a managed group
    async fn store_managed_group(&self, chat_id: ChatId, group_name: &str) -> Result<()>;

    /// Get all managed groups
    async fn get_managed_groups(&self) -> Result<Vec<ManagedGroup>>;

    /// Remove a managed group
    async fn remove_managed_group(&self, chat_id: ChatId) -> Result<bool>;

    // =========================================================================
    // FORWARDED MESSAGES OPERATIONS
    // =========================================================================

    /// Store a forwarded message mapping
    async fn store_forwarded_message(
        &self,
        admin_msg_id: MessageId,
        original_chat_id: ChatId,
        original_msg_id: MessageId,
    ) -> Result<()>;

    /// Get original message info from admin message ID
    async fn get_original_message(&self, admin_msg_id: MessageId) -> Result<Option<(ChatId, MessageId)>>;

    /// Get recent forwarded messages (for loading into memory on startup)
    async fn get_recent_forwarded_messages(&self, limit: i32) -> Result<Vec<ForwardedMessage>>;

    /// Clean up old forwarded messages (keep only recent ones)
    async fn cleanup_old_forwarded_messages(&self, keep_days: i32) -> Result<u64>;

    // =========================================================================
    // USER SESSION OPERATIONS
    // =========================================================================

    /// Update user AI preference
    async fn set_user_ai_enabled(&self, user_id: UserId, enabled: bool) -> Result<()>;

    /// Get user AI preference
    async fn get_user_ai_enabled(&self, user_id: UserId) -> Result<bool>;

    // =========================================================================
    // UTILITY OPERATIONS
    // =========================================================================

    /// Get database statistics
    async fn get_stats(&self) -> Result<DatabaseStats>;

    /// Health check - verify database connectivity
    async fn health_check(&self) -> Result<()>;
}