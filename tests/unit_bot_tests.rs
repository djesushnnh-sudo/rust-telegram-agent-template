use telegram_bot_template::config::Config;
use telegram_bot_template::error::{BotError, ErrorRecovery};

/// Unit tests for bot service initialization and basic operations
/// 
/// These tests focus on specific scenarios for bot service creation,
/// configuration handling, and error recovery mechanisms.

/// Helper function to create a test configuration
fn create_test_config() -> Config {
    Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: Some(8080),
        ai_enabled: false,
    }
}

/// Helper function to create a webhook config
fn create_webhook_config() -> Config {
    Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "debug".to_string(),
        webhook_url: Some("https://example.com/webhook".to_string()),
        port: Some(9000),
        ai_enabled: true,
    }
}

#[test]
fn test_config_access_methods() {
    let config = create_test_config();
    
    // Test config helper methods
    assert_eq!(config.get_port(), 8080);
    assert!(!config.is_webhook_mode());
    assert_eq!(config.get_webhook_url(), None);
    assert!(!config.ai_enabled);
    
    let webhook_config = create_webhook_config();
    assert_eq!(webhook_config.get_port(), 9000);
    assert!(webhook_config.is_webhook_mode());
    assert_eq!(webhook_config.get_webhook_url(), Some("https://example.com/webhook"));
    assert!(webhook_config.ai_enabled);
}

#[test]
fn test_error_recovery_classification() {
    // Test recoverable errors
    let command_error = BotError::Command("Invalid command syntax".to_string());
    assert!(ErrorRecovery::is_recoverable(&command_error));
    
    let ai_error = BotError::AI("AI service unavailable".to_string());
    assert!(ErrorRecovery::is_recoverable(&ai_error));
    
    let network_error = BotError::Network("Connection timeout".to_string());
    assert!(ErrorRecovery::is_recoverable(&network_error));
    
    let parsing_error = BotError::MessageParsing("Invalid message format".to_string());
    assert!(ErrorRecovery::is_recoverable(&parsing_error));
    
    // Test non-recoverable errors
    let config_error = BotError::Config(
        telegram_bot_template::error::ConfigError::MissingVariable {
            variable: "TELEGRAM_BOT_TOKEN".to_string(),
        }
    );
    assert!(!ErrorRecovery::is_recoverable(&config_error));
    
    let internal_error = BotError::Internal("Critical system failure".to_string());
    assert!(!ErrorRecovery::is_recoverable(&internal_error));
}

#[test]
fn test_user_friendly_error_messages() {
    let command_error = BotError::Command("Unknown command: /invalid".to_string());
    let message = ErrorRecovery::user_friendly_message(&command_error);
    assert!(message.contains("Sorry"));
    assert!(message.contains("command"));
    assert!(!message.is_empty());
    
    let ai_error = BotError::AI("Model not available".to_string());
    let message = ErrorRecovery::user_friendly_message(&ai_error);
    assert!(message.contains("AI"));
    assert!(message.contains("temporarily unavailable"));
    
    let network_error = BotError::Network("Connection failed".to_string());
    let message = ErrorRecovery::user_friendly_message(&network_error);
    assert!(message.contains("trouble connecting"));
    
    let parsing_error = BotError::MessageParsing("Invalid format".to_string());
    let message = ErrorRecovery::user_friendly_message(&parsing_error);
    assert!(message.contains("couldn't understand"));
    
    let internal_error = BotError::Internal("System error".to_string());
    let message = ErrorRecovery::user_friendly_message(&internal_error);
    assert!(message.contains("something went wrong"));
}

#[test]
fn test_error_display_formatting() {
    let config_error = BotError::Config(
        telegram_bot_template::error::ConfigError::InvalidValue {
            key: "port".to_string(),
            value: "invalid".to_string(),
            reason: "Must be a number".to_string(),
        }
    );
    
    let error_string = config_error.to_string();
    assert!(error_string.contains("Configuration error"));
    assert!(error_string.contains("port"));
    assert!(error_string.contains("invalid"));
    assert!(error_string.contains("Must be a number"));
}

#[test]
fn test_config_validation_edge_cases() {
    // Test empty bot token
    let mut config = create_test_config();
    config.bot_token = String::new();
    assert!(config.validate().is_err());
    
    // Test bot token without colon
    config.bot_token = "invalid_token_format".to_string();
    assert!(config.validate().is_err());
    
    // Test invalid log level
    config = create_test_config();
    config.log_level = "invalid_level".to_string();
    assert!(config.validate().is_err());
    
    // Test invalid webhook URL
    config = create_test_config();
    config.webhook_url = Some("not_a_url".to_string());
    assert!(config.validate().is_err());
    
    // Test port too low
    config = create_test_config();
    config.port = Some(100);
    assert!(config.validate().is_err());
    
    // Test valid configuration
    config = create_test_config();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_default_values() {
    let config = Config::default();
    
    assert!(config.bot_token.is_empty());
    assert_eq!(config.log_level, "info");
    assert_eq!(config.webhook_url, None);
    assert_eq!(config.port, Some(8080));
    assert!(!config.ai_enabled);
    
    // Test helper methods with defaults
    assert_eq!(config.get_port(), 8080);
    assert!(!config.is_webhook_mode());
    assert_eq!(config.get_webhook_url(), None);
}

#[test]
fn test_config_webhook_mode_detection() {
    let mut config = create_test_config();
    
    // No webhook URL
    assert!(!config.is_webhook_mode());
    assert_eq!(config.get_webhook_url(), None);
    
    // With webhook URL
    config.webhook_url = Some("https://example.com/webhook".to_string());
    assert!(config.is_webhook_mode());
    assert_eq!(config.get_webhook_url(), Some("https://example.com/webhook"));
}

#[test]
fn test_error_macro_functionality() {
    // Test command_error! macro
    let error = telegram_bot_template::command_error!("Test error message");
    match error {
        BotError::Command(msg) => assert_eq!(msg, "Test error message"),
        _ => panic!("Expected Command error"),
    }
    
    // Test formatted command_error! macro
    let error = telegram_bot_template::command_error!("Error code: {}", 404);
    match error {
        BotError::Command(msg) => assert_eq!(msg, "Error code: 404"),
        _ => panic!("Expected Command error"),
    }
    
    // Test ai_error! macro
    let error = telegram_bot_template::ai_error!("AI processing failed");
    match error {
        BotError::AI(msg) => assert_eq!(msg, "AI processing failed"),
        _ => panic!("Expected AI error"),
    }
    
    // Test internal_error! macro
    let error = telegram_bot_template::internal_error!("System failure");
    match error {
        BotError::Internal(msg) => assert_eq!(msg, "System failure"),
        _ => panic!("Expected Internal error"),
    }
}

#[test]
fn test_config_clone_and_equality() {
    let config1 = create_test_config();
    let config2 = config1.clone();
    
    assert_eq!(config1, config2);
    assert_eq!(config1.bot_token, config2.bot_token);
    assert_eq!(config1.log_level, config2.log_level);
    assert_eq!(config1.webhook_url, config2.webhook_url);
    assert_eq!(config1.port, config2.port);
    assert_eq!(config1.ai_enabled, config2.ai_enabled);
}

#[test]
fn test_config_serialization_fields() {
    // Test that Config has the expected fields for serialization
    let config = create_test_config();
    
    // These should not panic - testing that fields exist
    let _ = &config.bot_token;
    let _ = &config.log_level;
    let _ = &config.webhook_url;
    let _ = &config.port;
    let _ = &config.ai_enabled;
}

#[test]
fn test_error_chain_and_source() {
    use std::error::Error;
    
    let config_error = telegram_bot_template::error::ConfigError::MissingVariable {
        variable: "TEST_VAR".to_string(),
    };
    let bot_error = BotError::Config(config_error);
    
    // Test error chain
    assert!(bot_error.source().is_some());
    
    // Test error display
    let error_string = bot_error.to_string();
    assert!(error_string.contains("Configuration error"));
    assert!(error_string.contains("TEST_VAR"));
}

#[test]
fn test_port_boundary_values() {
    let mut config = create_test_config();
    
    // Test minimum valid port (1024)
    config.port = Some(1024);
    assert!(config.validate().is_ok());
    
    // Test just below minimum (1023)
    config.port = Some(1023);
    assert!(config.validate().is_err());
    
    // Test high port number
    config.port = Some(65535);
    assert!(config.validate().is_ok());
    
    // Test None (should use default)
    config.port = None;
    assert!(config.validate().is_ok());
    assert_eq!(config.get_port(), 8080); // Default
}

#[test]
fn test_log_level_validation() {
    let mut config = create_test_config();
    
    // Test all valid log levels
    let valid_levels = ["debug", "info", "warn", "error"];
    for level in valid_levels {
        config.log_level = level.to_string();
        assert!(config.validate().is_ok(), "Level '{}' should be valid", level);
    }
    
    // Test invalid log levels
    let invalid_levels = ["trace", "verbose", "critical", "fatal", ""];
    for level in invalid_levels {
        config.log_level = level.to_string();
        assert!(config.validate().is_err(), "Level '{}' should be invalid", level);
    }
}

#[test]
fn test_webhook_url_validation() {
    let mut config = create_test_config();
    
    // Test valid webhook URLs
    let valid_urls = [
        "https://example.com/webhook",
        "http://localhost:8080/webhook",
        "https://api.example.com/bot/webhook?token=abc",
    ];
    
    for url in valid_urls {
        config.webhook_url = Some(url.to_string());
        assert!(config.validate().is_ok(), "URL '{}' should be valid", url);
    }
    
    // Test invalid webhook URLs
    let invalid_urls = [
        "ftp://example.com/webhook",
        "example.com/webhook",
        "not_a_url",
        "",
    ];
    
    for url in invalid_urls {
        config.webhook_url = Some(url.to_string());
        assert!(config.validate().is_err(), "URL '{}' should be invalid", url);
    }
    
    // Test None webhook URL (should be valid)
    config.webhook_url = None;
    assert!(config.validate().is_ok());
}