use thiserror::Error;

/// Main error type for the Telegram bot template
/// 
/// This enum covers all error categories that can occur during bot operation,
/// providing structured error handling and clear error messages for debugging.
#[derive(Debug, Error)]
pub enum BotError {
    /// Configuration-related errors (missing tokens, invalid settings, etc.)
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    /// Telegram API communication errors
    #[error("Telegram API error: {0}")]
    Telegram(#[from] teloxide::RequestError),
    
    /// Command processing and execution errors
    #[error("Command error: {0}")]
    Command(String),
    
    /// AI processing and routing errors
    #[error("AI processing error: {0}")]
    AI(String),
    
    /// Network and connectivity errors
    #[error("Network error: {0}")]
    Network(String),
    
    /// Message parsing and validation errors
    #[error("Message parsing error: {0}")]
    MessageParsing(String),
    
    /// Internal application errors
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Configuration-specific error types
/// 
/// These errors occur during application startup and configuration loading.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Missing required environment variable
    #[error("Missing required environment variable: {variable}")]
    MissingVariable { variable: String },
    
    /// Invalid configuration value
    #[error("Invalid configuration value for {key}: {value} - {reason}")]
    InvalidValue {
        key: String,
        value: String,
        reason: String,
    },
    
    /// Environment variable parsing error
    #[error("Failed to parse environment variable {variable}: {source}")]
    ParseError {
        variable: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    /// Configuration validation error
    #[error("Configuration validation failed: {0}")]
    Validation(String),
}

/// Result type alias for bot operations
pub type BotResult<T> = Result<T, BotError>;

/// Result type alias for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Error recovery utilities for graceful error handling
pub struct ErrorRecovery;

impl ErrorRecovery {
    /// Determines if an error is recoverable and the bot should continue operating
    pub fn is_recoverable(error: &BotError) -> bool {
        match error {
            BotError::Config(_) => false, // Configuration errors are fatal
            BotError::Telegram(req_err) => {
                // Some Telegram API errors are recoverable (rate limits, temporary issues)
                Self::is_telegram_error_recoverable(req_err)
            }
            BotError::Command(_) => true, // Command errors shouldn't crash the bot
            BotError::AI(_) => true,      // AI errors should fall back to normal processing
            BotError::Network(_) => true, // Network errors are often temporary
            BotError::MessageParsing(_) => true, // Parsing errors for individual messages
            BotError::Internal(_) => false, // Internal errors might indicate serious issues
        }
    }
    
    /// Determines if a Telegram API error is recoverable
    fn is_telegram_error_recoverable(error: &teloxide::RequestError) -> bool {
        match error {
            teloxide::RequestError::Network(_) => true, // Network issues are temporary
            teloxide::RequestError::Api(api_error) => {
                // Check if the error message indicates a recoverable condition
                let error_msg = api_error.to_string().to_lowercase();
                error_msg.contains("too many requests") || 
                error_msg.contains("internal server error") ||
                error_msg.contains("bad gateway") ||
                error_msg.contains("service unavailable")
            }
            _ => false,
        }
    }
    
    /// Logs an error with appropriate level based on severity
    pub fn log_error(error: &BotError) {
        match error {
            BotError::Config(_) => log::error!("Fatal configuration error: {}", error),
            BotError::Internal(_) => log::error!("Internal error: {}", error),
            BotError::Telegram(_) => {
                if Self::is_recoverable(error) {
                    log::warn!("Recoverable Telegram API error: {}", error);
                } else {
                    log::error!("Fatal Telegram API error: {}", error);
                }
            }
            BotError::Command(_) => log::warn!("Command processing error: {}", error),
            BotError::AI(_) => log::warn!("AI processing error: {}", error),
            BotError::Network(_) => log::warn!("Network error: {}", error),
            BotError::MessageParsing(_) => log::debug!("Message parsing error: {}", error),
        }
    }
    
    /// Creates a user-friendly error message for display
    pub fn user_friendly_message(error: &BotError) -> String {
        match error {
            BotError::Command(msg) => format!("Sorry, there was an issue with that command: {}", msg),
            BotError::AI(_) => "AI processing is temporarily unavailable. Please try again later.".to_string(),
            BotError::Network(_) => "I'm having trouble connecting right now. Please try again in a moment.".to_string(),
            BotError::MessageParsing(_) => "I couldn't understand that message. Please try rephrasing it.".to_string(),
            _ => "Sorry, something went wrong. Please try again later.".to_string(),
        }
    }
}

/// Logging configuration utilities
pub struct LoggingConfig;

impl LoggingConfig {
    /// Initialize logging with environment-based configuration
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize env_logger with default configuration
        // This will read RUST_LOG environment variable for log level control
        env_logger::Builder::from_default_env()
            .format_timestamp_secs()
            .format_module_path(false)
            .init();
        
        log::info!("Logging initialized successfully");
        Ok(())
    }
    
    /// Initialize logging with custom log level
    pub fn init_with_level(level: log::LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
        env_logger::Builder::new()
            .filter_level(level)
            .format_timestamp_secs()
            .format_module_path(false)
            .init();
        
        log::info!("Logging initialized with level: {}", level);
        Ok(())
    }
}

/// Convenience macros for error creation
#[macro_export]
macro_rules! command_error {
    ($msg:expr) => {
        BotError::Command($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        BotError::Command(format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! ai_error {
    ($msg:expr) => {
        BotError::AI($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        BotError::AI(format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        BotError::Internal($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        BotError::Internal(format!($fmt, $($arg)*))
    };
}

// Re-export commonly used types
pub use BotError::*;
pub use ConfigError::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let config_error = BotError::Config(ConfigError::MissingVariable {
            variable: "TELEGRAM_BOT_TOKEN".to_string(),
        });
        
        assert!(config_error.to_string().contains("Configuration error"));
        assert!(config_error.to_string().contains("TELEGRAM_BOT_TOKEN"));
    }
    
    #[test]
    fn test_error_recovery_recoverable() {
        let command_error = BotError::Command("Invalid command".to_string());
        assert!(ErrorRecovery::is_recoverable(&command_error));
        
        let config_error = BotError::Config(ConfigError::MissingVariable {
            variable: "TOKEN".to_string(),
        });
        assert!(!ErrorRecovery::is_recoverable(&config_error));
    }
    
    #[test]
    fn test_user_friendly_messages() {
        let command_error = BotError::Command("Test error".to_string());
        let message = ErrorRecovery::user_friendly_message(&command_error);
        assert!(message.contains("Sorry"));
        assert!(!message.is_empty());
    }
    
    #[test]
    fn test_error_macros() {
        let error = command_error!("Test command error");
        match error {
            BotError::Command(msg) => assert_eq!(msg, "Test command error"),
            _ => panic!("Wrong error type"),
        }
        
        let formatted_error = ai_error!("AI error: {}", "processing failed");
        match formatted_error {
            BotError::AI(msg) => assert_eq!(msg, "AI error: processing failed"),
            _ => panic!("Wrong error type"),
        }
    }
}