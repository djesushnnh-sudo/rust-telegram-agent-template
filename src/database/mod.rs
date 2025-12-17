use anyhow::Result;
use std::sync::Arc;

// Re-export the main components
pub mod models;
pub mod migrations;
pub mod errors;
pub mod provider;
pub mod providers;
pub mod manager;

// Re-export commonly used types
pub use provider::{DatabaseProvider, DatabaseStats};
pub use manager::{DatabaseManager, DatabaseConfig};
pub use providers::{SqliteProvider, InMemoryProvider};

/// Legacy Database struct for backward compatibility
/// 
/// This is a wrapper around SqliteProvider to maintain backward compatibility
/// with existing code. New code should use DatabaseManager and DatabaseProvider trait.
/// 
/// # Deprecated
/// Use `DatabaseManager::create_provider()` with `DatabaseConfig::SQLite` instead.
pub struct Database {
    provider: Arc<dyn DatabaseProvider>,
}

impl Database {
    /// Create a new database connection (legacy method)
    /// 
    /// # Arguments
    /// * `database_url` - Database URL (e.g., "sqlite:bot.db")
    /// 
    /// # Example
    /// ```no_run
    /// use telegram_bot_template::database::Database;
    /// 
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let db = Database::new("sqlite:bot.db").await?;
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Deprecated
    /// Use `DatabaseManager::create_from_url()` instead for better flexibility.
    pub async fn new(database_url: &str) -> Result<Self> {
        let provider = DatabaseManager::create_from_url(database_url).await?;
        Ok(Database { provider })
    }
    
    /// Get the underlying provider (for migration to new API)
    pub fn provider(&self) -> Arc<dyn DatabaseProvider> {
        self.provider.clone()
    }
}

// Implement all the legacy methods by delegating to the provider
impl Database {
    pub async fn create_user(&self, username: &str, telegram_id: i64) -> Result<()> {
        self.provider.create_user(username, telegram_id).await
    }
    
    pub async fn get_user(&self, telegram_id: i64) -> Result<Option<models::User>> {
        self.provider.get_user(telegram_id).await
    }
    
    pub async fn get_all_users(&self) -> Result<Vec<models::User>> {
        self.provider.get_all_users().await
    }
    
    pub async fn delete_user(&self, telegram_id: i64) -> Result<bool> {
        self.provider.delete_user(telegram_id).await
    }

    pub async fn store_managed_group(&self, chat_id: teloxide::types::ChatId, group_name: &str) -> Result<()> {
        self.provider.store_managed_group(chat_id, group_name).await
    }

    pub async fn get_managed_groups(&self) -> Result<Vec<models::ManagedGroup>> {
        self.provider.get_managed_groups().await
    }

    pub async fn remove_managed_group(&self, chat_id: teloxide::types::ChatId) -> Result<bool> {
        self.provider.remove_managed_group(chat_id).await
    }

    pub async fn store_forwarded_message(
        &self,
        admin_msg_id: teloxide::types::MessageId,
        original_chat_id: teloxide::types::ChatId,
        original_msg_id: teloxide::types::MessageId,
    ) -> Result<()> {
        self.provider.store_forwarded_message(admin_msg_id, original_chat_id, original_msg_id).await
    }

    pub async fn get_original_message(&self, admin_msg_id: teloxide::types::MessageId) -> Result<Option<(teloxide::types::ChatId, teloxide::types::MessageId)>> {
        self.provider.get_original_message(admin_msg_id).await
    }

    pub async fn get_recent_forwarded_messages(&self, limit: i32) -> Result<Vec<models::ForwardedMessage>> {
        self.provider.get_recent_forwarded_messages(limit).await
    }

    pub async fn cleanup_old_forwarded_messages(&self, keep_days: i32) -> Result<u64> {
        self.provider.cleanup_old_forwarded_messages(keep_days).await
    }

    pub async fn set_user_ai_enabled(&self, user_id: teloxide::types::UserId, enabled: bool) -> Result<()> {
        self.provider.set_user_ai_enabled(user_id, enabled).await
    }

    pub async fn get_user_ai_enabled(&self, user_id: teloxide::types::UserId) -> Result<bool> {
        self.provider.get_user_ai_enabled(user_id).await
    }

    pub async fn get_stats(&self) -> Result<DatabaseStats> {
        self.provider.get_stats().await
    }

    pub async fn health_check(&self) -> Result<()> {
        self.provider.health_check().await
    }
}