use thiserror::Error;

/// Custom database errors for more granular error handling
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(#[from] sqlx::Error),
    
    #[error("User with Telegram ID {telegram_id} not found")]
    UserNotFound { telegram_id: i64 },
    
    #[error("User with Telegram ID {telegram_id} already exists")]
    UserAlreadyExists { telegram_id: i64 },
    
    #[error("Migration failed: {0}")]
    MigrationFailed(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;