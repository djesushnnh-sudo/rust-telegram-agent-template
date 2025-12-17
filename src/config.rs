use serde::{Deserialize, Serialize};
use std::env;
use crate::error::{ConfigError, ConfigResult};

/// Main configuration structure for the Telegram bot
/// 
/// This struct holds all configuration values loaded from environment variables.
/// It provides a centralized way to manage bot settings and feature flags.
/// 
/// ## Adding New Configuration Fields
/// 
/// When extending the bot with new features, follow this checklist:
/// 
/// 1. **Add the field** to this struct with proper documentation
/// 2. **Update `from_env()`** to load the value from environment variables
/// 3. **Update `validate()`** to validate the new field if needed
/// 4. **Update `.env.example`** with documentation and examples
/// 5. **Update `Default`** implementation if the field should have a default
/// 6. **Add tests** for the new configuration field
/// 
/// ### Common Configuration Patterns:
/// 
/// - **Optional API keys**: Use `Option<String>` for external service keys
/// - **Feature flags**: Use `bool` with sensible defaults for toggles  
/// - **Numeric limits**: Use appropriate integer types with validation
/// - **Database URLs**: Use `Option<String>` for optional database connections
/// 
/// ### Environment Variable Naming Convention:
/// - Use SCREAMING_SNAKE_CASE for environment variables
/// - Prefix bot-specific variables with `BOT_` if needed
/// - Use descriptive names: `WEATHER_API_KEY` not `API_KEY`
/// - Boolean flags: `ENABLE_FEATURE` not `FEATURE_ENABLED`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Telegram bot token from BotFather (required)
    pub bot_token: String,
    
    /// Logging level (debug, info, warn, error)
    pub log_level: String,
    
    /// Optional webhook URL for webhook mode
    pub webhook_url: Option<String>,
    
    /// Port for webhook server (default: 8080)
    pub port: Option<u16>,
    
    /// Enable AI processing (default: false)
    pub ai_enabled: bool,
    
    /// Database URL for persistent storage
    /// Supports: sqlite:path/to/db.db, memory:, none:
    pub database_url: Option<String>,
    
    /// Database provider type (sqlite, memory, none)
    /// If not specified, will be inferred from database_url
    pub database_provider: Option<String>,
}

impl Config {
    /// Load configuration from environment variables
    /// 
    /// This method loads the .env file and then reads configuration from environment variables.
    pub fn from_env() -> ConfigResult<Self> {
        // Load .env file (simple approach for template)
        println!("DEBUG: Loading .env file");
        if let Err(e) = dotenv::dotenv() {
            println!("DEBUG: Failed to load .env: {}", e);
            log::warn!("⚠️ No .env file found, using system environment variables only");
        } else {
            println!("DEBUG: Successfully loaded .env");
        }
        let bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .map_err(|_| ConfigError::MissingVariable { 
                variable: "TELEGRAM_BOT_TOKEN".to_string() 
            })?;
        
        // Debug: Print token info (without revealing the actual token)
        println!("DEBUG: Token length: {}", bot_token.len());
        println!("DEBUG: Token contains colon: {}", bot_token.contains(':'));
        println!("DEBUG: Token first 10 chars: {}", &bot_token[..std::cmp::min(10, bot_token.len())]);

        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        let webhook_url = env::var("BOT_WEBHOOK_URL").ok()
            .filter(|url| !url.is_empty());

        let port = env::var("BOT_PORT")
            .ok()
            .map(|p| p.parse::<u16>())
            .transpose()
            .map_err(|e| ConfigError::ParseError {
                variable: "BOT_PORT".to_string(),
                source: Box::new(e),
            })?;

        let ai_enabled = env::var("AI_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .map_err(|e| ConfigError::ParseError {
                variable: "AI_ENABLED".to_string(),
                source: Box::new(e),
            })?;

        let database_url = env::var("DATABASE_URL").ok();
        let database_provider = env::var("DATABASE_PROVIDER").ok();

        let config = Config {
            bot_token,
            log_level,
            webhook_url,
            port,
            ai_enabled,
            database_url,
            database_provider,
        };

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration values
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate bot token format (should start with a number followed by colon)
        if self.bot_token.is_empty() {
            return Err(ConfigError::Validation(
                "Bot token cannot be empty".to_string()
            ));
        }

        // Basic bot token format validation
        if !self.bot_token.contains(':') {
            return Err(ConfigError::InvalidValue {
                key: "bot_token".to_string(),
                value: "[REDACTED]".to_string(),
                reason: "Bot token should contain a colon (:)".to_string(),
            });
        }

        // Validate log level
        let valid_log_levels = ["debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&self.log_level.as_str()) {
            return Err(ConfigError::InvalidValue {
                key: "log_level".to_string(),
                value: self.log_level.clone(),
                reason: format!("Must be one of: {}", valid_log_levels.join(", ")),
            });
        }

        // Validate webhook URL format if provided
        if let Some(ref url) = self.webhook_url {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(ConfigError::InvalidValue {
                    key: "webhook_url".to_string(),
                    value: url.clone(),
                    reason: "Must start with http:// or https://".to_string(),
                });
            }
        }

        // Validate port range if provided
        if let Some(port) = self.port {
            if port < 1024 {
                return Err(ConfigError::InvalidValue {
                    key: "port".to_string(),
                    value: port.to_string(),
                    reason: "Port must be 1024 or higher".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Get the effective port (default to 8080 if not specified)
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }

    /// Check if webhook mode is enabled
    pub fn is_webhook_mode(&self) -> bool {
        self.webhook_url.is_some()
    }

    /// Get webhook URL if configured
    pub fn get_webhook_url(&self) -> Option<&str> {
        self.webhook_url.as_deref()
    }
    
    /// Get database configuration
    pub fn get_database_config(&self) -> crate::database::DatabaseConfig {
        use crate::database::DatabaseConfig;
        
        // If database_provider is explicitly set, use it
        if let Some(ref provider) = self.database_provider {
            match provider.as_str() {
                "sqlite" => {
                    let url = self.database_url.as_deref().unwrap_or("sqlite:bot.db");
                    DatabaseConfig::SQLite { database_url: url.to_string() }
                }
                "memory" => DatabaseConfig::InMemory,
                "none" => DatabaseConfig::None,
                _ => {
                    log::warn!("Unknown database provider '{}', defaulting to SQLite", provider);
                    DatabaseConfig::default()
                }
            }
        }
        // Otherwise, infer from database_url
        else if let Some(ref url) = self.database_url {
            if url.starts_with("sqlite:") {
                DatabaseConfig::SQLite { database_url: url.clone() }
            } else if url.starts_with("memory:") || url.starts_with("mem:") {
                DatabaseConfig::InMemory
            } else if url.starts_with("none:") || url.starts_with("noop:") {
                DatabaseConfig::None
            } else {
                // Assume SQLite for backward compatibility
                DatabaseConfig::SQLite { database_url: format!("sqlite:{}", url) }
            }
        }
        // Default to SQLite
        else {
            DatabaseConfig::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bot_token: String::new(),
            log_level: "info".to_string(),
            webhook_url: None,
            port: Some(8080),
            ai_enabled: false,
            database_url: None,
            database_provider: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_config_validation_valid() {
        let config = Config {
            bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
            log_level: "info".to_string(),
            webhook_url: Some("https://example.com/webhook".to_string()),
            port: Some(8080),
            ai_enabled: false,
            database_url: None,
            database_provider: None,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_bot_token() {
        let config = Config {
            bot_token: "invalid_token".to_string(),
            log_level: "info".to_string(),
            webhook_url: None,
            port: Some(8080),
            ai_enabled: false,
            database_url: None,
            database_provider: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_log_level() {
        let config = Config {
            bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
            log_level: "invalid".to_string(),
            webhook_url: None,
            port: Some(8080),
            ai_enabled: false,
            database_url: None,
            database_provider: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_webhook_url() {
        let config = Config {
            bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
            log_level: "info".to_string(),
            webhook_url: Some("invalid_url".to_string()),
            port: Some(8080),
            ai_enabled: false,
            database_url: None,
            database_provider: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_port() {
        let config = Config {
            bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
            log_level: "info".to_string(),
            webhook_url: None,
            port: Some(100), // Invalid port (too low)
            ai_enabled: false,
            database_url: None,
            database_provider: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.get_port(), 8080);
        assert!(!config.ai_enabled);
        assert!(!config.is_webhook_mode());
    }

    #[test]
    fn test_webhook_mode_detection() {
        let mut config = Config::default();
        assert!(!config.is_webhook_mode());

        config.webhook_url = Some("https://example.com/webhook".to_string());
        assert!(config.is_webhook_mode());
    }
}