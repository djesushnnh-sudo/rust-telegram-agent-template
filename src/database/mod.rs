use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Row};
use serde::{Deserialize, Serialize};
use log::{info, debug};
use teloxide::types::{ChatId, UserId, MessageId};
use chrono::Utc;

pub mod models;
pub mod migrations;
pub mod errors;

use models::{User, ManagedGroup, ForwardedMessage};
use migrations::Migrator;

/// Enhanced SQLite database adapter with state management features
/// 
/// This database adapter provides persistent storage for bot state,
/// including user sessions, managed groups, and message mappings.
/// Based on patterns from Dads2Dads but simplified for SQLite.
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection
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
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connecting to database: {}", database_url);
        
        let pool = SqlitePool::connect(database_url).await?;
        
        let db = Database { pool: pool.clone() };
        
        // Run migrations instead of init_tables
        let migrator = Migrator::new(pool);
        migrator.migrate().await?;
        
        info!("Database initialized successfully");
        Ok(db)
    }
    

    
    /// Create a new user
    /// 
    /// # Arguments
    /// * `username` - The user's username
    /// * `telegram_id` - The user's Telegram ID
    pub async fn create_user(&self, username: &str, telegram_id: i64) -> Result<()> {
        debug!("Creating/updating user: {} (ID: {})", username, telegram_id);
        
        sqlx::query(
            "INSERT OR REPLACE INTO users (username, telegram_id) VALUES (?, ?)"
        )
        .bind(username)
        .bind(telegram_id)
        .execute(&self.pool)
        .await?;
        
        debug!("User created/updated successfully");
        Ok(())
    }
    
    /// Get a user by their Telegram ID
    /// 
    /// # Arguments
    /// * `telegram_id` - The user's Telegram ID
    /// 
    /// # Returns
    /// * `Some(User)` if found, `None` if not found
    pub async fn get_user(&self, telegram_id: i64) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, telegram_id, username, created_at FROM users WHERE telegram_id = ?"
        )
        .bind(telegram_id)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    telegram_id: row.get("telegram_id"),
                    username: row.get("username"),
                    created_at: row.get("created_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
    
    /// Get all users
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let rows = sqlx::query(
            "SELECT id, telegram_id, username, created_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.get("id"),
                telegram_id: row.get("telegram_id"),
                username: row.get("username"),
                created_at: row.get("created_at"),
            })
            .collect();
        
        Ok(users)
    }
    
    /// Delete a user by their Telegram ID
    pub async fn delete_user(&self, telegram_id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM users WHERE telegram_id = ?")
            .bind(telegram_id)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // MANAGED GROUPS OPERATIONS (Based on Dads2Dads patterns)
    // =========================================================================

    /// Store a managed group
    pub async fn store_managed_group(&self, chat_id: ChatId, group_name: &str) -> Result<()> {
        debug!("Storing managed group: {} ({})", group_name, chat_id);
        
        sqlx::query(
            "INSERT OR REPLACE INTO managed_groups (chat_id, group_name, updated_at) VALUES (?, ?, ?)"
        )
        .bind(chat_id.0)
        .bind(group_name)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    /// Get all managed groups
    pub async fn get_managed_groups(&self) -> Result<Vec<ManagedGroup>> {
        let rows = sqlx::query(
            "SELECT chat_id, group_name, created_at, updated_at FROM managed_groups ORDER BY group_name"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let groups = rows
            .into_iter()
            .map(|row| ManagedGroup {
                chat_id: ChatId(row.get("chat_id")),
                group_name: row.get("group_name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();
        
        Ok(groups)
    }

    /// Remove a managed group
    pub async fn remove_managed_group(&self, chat_id: ChatId) -> Result<bool> {
        let result = sqlx::query("DELETE FROM managed_groups WHERE chat_id = ?")
            .bind(chat_id.0)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // FORWARDED MESSAGES OPERATIONS (Based on Dads2Dads patterns)
    // =========================================================================

    /// Store a forwarded message mapping
    pub async fn store_forwarded_message(
        &self,
        admin_msg_id: MessageId,
        original_chat_id: ChatId,
        original_msg_id: MessageId,
    ) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO forwarded_messages (admin_msg_id, original_chat_id, original_msg_id, created_at) VALUES (?, ?, ?, ?)"
        )
        .bind(admin_msg_id.0)
        .bind(original_chat_id.0)
        .bind(original_msg_id.0)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    /// Get original message info from admin message ID
    pub async fn get_original_message(&self, admin_msg_id: MessageId) -> Result<Option<(ChatId, MessageId)>> {
        let row = sqlx::query(
            "SELECT original_chat_id, original_msg_id FROM forwarded_messages WHERE admin_msg_id = ?"
        )
        .bind(admin_msg_id.0)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let original_chat_id = ChatId(row.get("original_chat_id"));
                let original_msg_id = MessageId(row.get("original_msg_id"));
                Ok(Some((original_chat_id, original_msg_id)))
            }
            None => Ok(None),
        }
    }

    /// Get recent forwarded messages (for loading into memory on startup)
    pub async fn get_recent_forwarded_messages(&self, limit: i32) -> Result<Vec<ForwardedMessage>> {
        let rows = sqlx::query(
            "SELECT admin_msg_id, original_chat_id, original_msg_id, created_at 
             FROM forwarded_messages 
             ORDER BY created_at DESC 
             LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        let messages = rows
            .into_iter()
            .map(|row| ForwardedMessage {
                admin_msg_id: MessageId(row.get("admin_msg_id")),
                original_chat_id: ChatId(row.get("original_chat_id")),
                original_msg_id: MessageId(row.get("original_msg_id")),
                created_at: row.get("created_at"),
            })
            .collect();
        
        Ok(messages)
    }

    /// Clean up old forwarded messages (keep only recent ones)
    pub async fn cleanup_old_forwarded_messages(&self, keep_days: i32) -> Result<u64> {
        let cutoff = Utc::now() - chrono::Duration::days(keep_days as i64);
        
        let result = sqlx::query(
            "DELETE FROM forwarded_messages WHERE created_at < ?"
        )
        .bind(cutoff)
        .execute(&self.pool)
        .await?;
        
        let deleted_count = result.rows_affected();
        if deleted_count > 0 {
            info!("Cleaned up {} old forwarded messages", deleted_count);
        }
        
        Ok(deleted_count)
    }

    // =========================================================================
    // USER SESSION OPERATIONS (For AI routing)
    // =========================================================================

    /// Update user AI preference
    pub async fn set_user_ai_enabled(&self, user_id: UserId, enabled: bool) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO user_preferences (user_id, ai_enabled, updated_at) VALUES (?, ?, ?)"
        )
        .bind(user_id.0 as i64)
        .bind(enabled)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    /// Get user AI preference
    pub async fn get_user_ai_enabled(&self, user_id: UserId) -> Result<bool> {
        let row = sqlx::query(
            "SELECT ai_enabled FROM user_preferences WHERE user_id = ?"
        )
        .bind(user_id.0 as i64)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(row.get("ai_enabled")),
            None => Ok(false), // Default to AI disabled
        }
    }

    // =========================================================================
    // UTILITY OPERATIONS
    // =========================================================================

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<DatabaseStats> {
        let users_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
        
        let groups_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM managed_groups")
            .fetch_one(&self.pool)
            .await?;
        
        let messages_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM forwarded_messages")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(DatabaseStats {
            users_count: users_count as u64,
            groups_count: groups_count as u64,
            forwarded_messages_count: messages_count as u64,
        })
    }

    /// Health check - verify database connectivity
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }
}

/// Database statistics for monitoring
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub users_count: u64,
    pub groups_count: u64,
    pub forwarded_messages_count: u64,
}