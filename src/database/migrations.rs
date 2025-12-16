use anyhow::Result;
use sqlx::{SqlitePool, Row};
use log::{info, debug};

/// Simple migration system for database schema changes
pub struct Migrator {
    pool: SqlitePool,
}

impl Migrator {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Run all pending migrations
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations");
        
        // Create migrations table to track applied migrations
        self.create_migrations_table().await?;
        
        // Get current schema version
        let current_version = self.get_current_version().await?;
        debug!("Current database version: {}", current_version);
        
        // Apply migrations in order
        if current_version < 1 {
            self.migrate_to_v1().await?;
        }
        
        // Future migrations would go here:
        // if current_version < 2 {
        //     self.migrate_to_v2().await?;
        // }
        
        info!("Database migrations completed");
        Ok(())
    }
    
    /// Create the migrations tracking table
    async fn create_migrations_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get the current schema version
    async fn get_current_version(&self) -> Result<i32> {
        let row = sqlx::query("SELECT MAX(version) as version FROM schema_migrations")
            .fetch_optional(&self.pool)
            .await?;
            
        match row {
            Some(row) => {
                let version: Option<i32> = row.get("version");
                Ok(version.unwrap_or(0))
            }
            None => Ok(0),
        }
    }
    
    /// Migration to version 1: Create initial tables with enhanced schema
    async fn migrate_to_v1(&self) -> Result<()> {
        debug!("Applying migration v1: Enhanced schema with state management");
        
        // Create users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                telegram_id INTEGER UNIQUE NOT NULL,
                username TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create managed_groups table (based on Dads2Dads patterns)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS managed_groups (
                chat_id INTEGER PRIMARY KEY,
                group_name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create forwarded_messages table (based on Dads2Dads patterns)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS forwarded_messages (
                admin_msg_id INTEGER PRIMARY KEY,
                original_chat_id INTEGER NOT NULL,
                original_msg_id INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create user_preferences table (for AI settings)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_preferences (
                user_id INTEGER PRIMARY KEY,
                ai_enabled BOOLEAN DEFAULT FALSE,
                language TEXT DEFAULT 'en',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        
        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_telegram_id ON users(telegram_id)")
            .execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_forwarded_messages_original ON forwarded_messages(original_chat_id, original_msg_id)")
            .execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_forwarded_messages_created ON forwarded_messages(created_at)")
            .execute(&self.pool).await?;
        
        // Mark migration as applied
        sqlx::query("INSERT INTO schema_migrations (version) VALUES (1)")
            .execute(&self.pool)
            .await?;
            
        info!("Migration v1 applied successfully - Enhanced schema with state management");
        Ok(())
    }
}