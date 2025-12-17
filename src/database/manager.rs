use anyhow::{Result, anyhow};
use std::sync::Arc;
use log::info;

use crate::database::provider::DatabaseProvider;
use crate::database::providers::{SqliteProvider, InMemoryProvider};

/// Database configuration for provider selection
#[derive(Debug, Clone)]
pub enum DatabaseConfig {
    /// SQLite database with file path
    SQLite { database_url: String },
    /// In-memory database (for testing/development)
    InMemory,
    /// No database (all operations are no-ops)
    None,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig::None
    }
}

/// Database manager that creates and manages database providers
/// 
/// This manager handles the creation of different database providers based on
/// configuration. It provides a unified interface for the rest of the application
/// to interact with the database without knowing which provider is being used.
pub struct DatabaseManager;

impl DatabaseManager {
    /// Create a database provider based on the configuration
    /// 
    /// # Arguments
    /// * `config` - Database configuration specifying which provider to use
    /// 
    /// # Returns
    /// * `Arc<dyn DatabaseProvider>` - Thread-safe database provider
    /// 
    /// # Example
    /// ```no_run
    /// use telegram_bot_template::database::{DatabaseManager, DatabaseConfig};
    /// 
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::SQLite {
    ///         database_url: "sqlite:bot.db".to_string(),
    ///     };
    ///     let db = DatabaseManager::create_provider(config).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_provider(config: DatabaseConfig) -> Result<Arc<dyn DatabaseProvider>> {
        match config {
            DatabaseConfig::SQLite { database_url } => {
                info!("Initializing SQLite database provider");
                let provider = SqliteProvider::new(&database_url).await?;
                Ok(Arc::new(provider))
            }
            DatabaseConfig::InMemory => {
                info!("Initializing in-memory database provider");
                let provider = InMemoryProvider::new();
                Ok(Arc::new(provider))
            }
            DatabaseConfig::None => {
                info!("Initializing no-op database provider");
                let provider = NoOpProvider::new();
                Ok(Arc::new(provider))
            }
        }
    }
    
    /// Create a provider from a database URL string
    /// 
    /// This is a convenience method that parses common database URL formats:
    /// - `sqlite:path/to/db.db` -> SQLite provider
    /// - `memory:` or `mem:` -> In-memory provider
    /// - `none:` or `noop:` -> No-op provider
    /// 
    /// # Arguments
    /// * `database_url` - Database URL string
    pub async fn create_from_url(database_url: &str) -> Result<Arc<dyn DatabaseProvider>> {
        let config = if database_url.starts_with("sqlite:") {
            DatabaseConfig::SQLite {
                database_url: database_url.to_string(),
            }
        } else if database_url.starts_with("memory:") || database_url.starts_with("mem:") {
            DatabaseConfig::InMemory
        } else if database_url.starts_with("none:") || database_url.starts_with("noop:") {
            DatabaseConfig::None
        } else {
            return Err(anyhow!("Unsupported database URL format: {}", database_url));
        };
        
        Self::create_provider(config).await
    }
}

/// No-op database provider that does nothing
/// 
/// This provider implements all database operations as no-ops, returning
/// empty results or default values. It's useful when you want to disable
/// database functionality entirely.
pub struct NoOpProvider;

impl NoOpProvider {
    pub fn new() -> Self {
        info!("Initializing no-op database provider");
        Self
    }
}

impl Default for NoOpProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl DatabaseProvider for NoOpProvider {
    async fn create_user(&self, _username: &str, _telegram_id: i64) -> Result<()> {
        Ok(())
    }
    
    async fn get_user(&self, _telegram_id: i64) -> Result<Option<crate::database::models::User>> {
        Ok(None)
    }
    
    async fn get_all_users(&self) -> Result<Vec<crate::database::models::User>> {
        Ok(Vec::new())
    }
    
    async fn delete_user(&self, _telegram_id: i64) -> Result<bool> {
        Ok(false)
    }

    async fn store_managed_group(&self, _chat_id: teloxide::types::ChatId, _group_name: &str) -> Result<()> {
        Ok(())
    }

    async fn get_managed_groups(&self) -> Result<Vec<crate::database::models::ManagedGroup>> {
        Ok(Vec::new())
    }

    async fn remove_managed_group(&self, _chat_id: teloxide::types::ChatId) -> Result<bool> {
        Ok(false)
    }

    async fn store_forwarded_message(
        &self,
        _admin_msg_id: teloxide::types::MessageId,
        _original_chat_id: teloxide::types::ChatId,
        _original_msg_id: teloxide::types::MessageId,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_original_message(&self, _admin_msg_id: teloxide::types::MessageId) -> Result<Option<(teloxide::types::ChatId, teloxide::types::MessageId)>> {
        Ok(None)
    }

    async fn get_recent_forwarded_messages(&self, _limit: i32) -> Result<Vec<crate::database::models::ForwardedMessage>> {
        Ok(Vec::new())
    }

    async fn cleanup_old_forwarded_messages(&self, _keep_days: i32) -> Result<u64> {
        Ok(0)
    }

    async fn set_user_ai_enabled(&self, _user_id: teloxide::types::UserId, _enabled: bool) -> Result<()> {
        Ok(())
    }

    async fn get_user_ai_enabled(&self, _user_id: teloxide::types::UserId) -> Result<bool> {
        Ok(false)
    }

    async fn get_stats(&self) -> Result<crate::database::provider::DatabaseStats> {
        Ok(crate::database::provider::DatabaseStats {
            users_count: 0,
            groups_count: 0,
            forwarded_messages_count: 0,
        })
    }

    async fn health_check(&self) -> Result<()> {
        Ok(())
    }
}