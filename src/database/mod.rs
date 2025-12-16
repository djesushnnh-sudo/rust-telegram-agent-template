use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Row};
use serde::{Deserialize, Serialize};
use log::{info, debug};

pub mod models;
pub mod migrations;
pub mod errors;

use models::User;
use migrations::Migrator;

/// Simple SQLite database adapter for the Telegram bot
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
    /// ```
    /// let db = Database::new("sqlite:bot.db").await?;
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
}