use telegram_bot_template::config::Config;
use telegram_bot_template::error::ConfigError;

/// Unit tests for configuration loading edge cases
/// 
/// These tests focus on specific scenarios and edge cases for configuration
/// loading that complement the property-based tests.
/// 
/// Note: These tests focus on validation logic rather than environment variable loading
/// to avoid test interference issues.

// Note: Environment variable tests are skipped to avoid test interference
// The existing unit tests in src/config.rs already cover the validation logic

#[test]
fn test_config_validation_empty_token() {
    let config = Config {
        bot_token: String::new(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: None,
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_config_validation_token_without_colon() {
    let config = Config {
        bot_token: "invalid_token_format".to_string(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: None,
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
    
    if let Err(ConfigError::InvalidValue { key, reason, .. }) = result {
        assert_eq!(key, "bot_token");
        assert!(reason.contains("colon"));
    } else {
        panic!("Expected InvalidValue error for bot_token");
    }
}

#[test]
fn test_config_validation_invalid_log_level() {
    let config = Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "invalid_level".to_string(),
        webhook_url: None,
        port: None,
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
    
    if let Err(ConfigError::InvalidValue { key, value, .. }) = result {
        assert_eq!(key, "log_level");
        assert_eq!(value, "invalid_level");
    } else {
        panic!("Expected InvalidValue error for log_level");
    }
}

#[test]
fn test_config_validation_invalid_webhook_url_format() {
    let config = Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: Some("invalid_url_format".to_string()),
        port: None,
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
    
    if let Err(ConfigError::InvalidValue { key, .. }) = result {
        assert_eq!(key, "webhook_url");
    } else {
        panic!("Expected InvalidValue error for webhook_url");
    }
}

#[test]
fn test_config_validation_port_too_low() {
    let config = Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: Some(500), // Below 1024
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
    
    if let Err(ConfigError::InvalidValue { key, .. }) = result {
        assert_eq!(key, "port");
    } else {
        panic!("Expected InvalidValue error for port");
    }
}

#[test]
fn test_config_helper_methods() {
    let config = Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: Some("https://example.com/webhook".to_string()),
        port: Some(9000),
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    // Test helper methods
    assert_eq!(config.get_port(), 9000);
    assert!(config.is_webhook_mode());
    assert_eq!(config.get_webhook_url(), Some("https://example.com/webhook"));
    
    // Test with no port set
    let config_no_port = Config {
        port: None,
        ..config.clone()
    };
    assert_eq!(config_no_port.get_port(), 8080); // Default
    
    // Test with no webhook
    let config_no_webhook = Config {
        webhook_url: None,
        ..config
    };
    assert!(!config_no_webhook.is_webhook_mode());
    assert_eq!(config_no_webhook.get_webhook_url(), None);
}

#[test]
fn test_config_edge_case_whitespace_webhook() {
    // Test validation of whitespace-only webhook URL
    let config = Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: Some("   ".to_string()), // Whitespace only
        port: None,
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    // This should fail validation because whitespace-only URLs are invalid
    assert!(config.validate().is_err());
}

#[test]
fn test_config_boundary_port_values() {
    // Test port exactly at boundary (1024)
    let config = Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: Some(1024),
        ai_enabled: false,
        database_url: None,
        deploy_env: "dev".to_string(),
    };
    
    assert!(config.validate().is_ok());
    
    // Test port just below boundary (1023)
    let config_invalid = Config {
        port: Some(1023),
        ..config
    };
    
    assert!(config_invalid.validate().is_err());
}