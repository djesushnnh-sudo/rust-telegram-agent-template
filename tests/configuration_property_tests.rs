use quickcheck::{quickcheck, TestResult};
use std::env;
use telegram_bot_template::config::Config;

/// **Feature: telegram-bot-template, Property 4: Configuration Loading Reliability**
/// For any valid configuration provided through environment variables, 
/// the application should successfully load and apply all configuration settings
/// **Validates: Requirements 4.1, 4.4**
fn property_configuration_loading_reliability(
    bot_token: String,
    log_level: String,
    webhook_url: Option<String>,
    port: Option<u16>,
    ai_enabled: bool,
) -> TestResult {
    // Filter out invalid inputs to focus on valid configuration scenarios
    if bot_token.is_empty() || !bot_token.contains(':') {
        return TestResult::discard();
    }
    
    let valid_log_levels = ["debug", "info", "warn", "error"];
    if !valid_log_levels.contains(&log_level.as_str()) {
        return TestResult::discard();
    }
    
    // Validate webhook URL format if provided
    if let Some(ref url) = webhook_url {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return TestResult::discard();
        }
    }
    
    // Validate port range if provided
    if let Some(p) = port {
        if p < 1024 {
            return TestResult::discard();
        }
    }

    // Set up environment variables
    env::set_var("TELEGRAM_BOT_TOKEN", &bot_token);
    env::set_var("RUST_LOG", &log_level);
    
    if let Some(ref url) = webhook_url {
        env::set_var("BOT_WEBHOOK_URL", url);
    } else {
        env::remove_var("BOT_WEBHOOK_URL");
    }
    
    if let Some(p) = port {
        env::set_var("BOT_PORT", p.to_string());
    } else {
        env::remove_var("BOT_PORT");
    }
    
    env::set_var("AI_ENABLED", ai_enabled.to_string());

    // Test configuration loading
    match Config::from_env() {
        Ok(config) => {
            // Verify all settings were loaded correctly
            TestResult::from_bool(
                config.bot_token == bot_token
                    && config.log_level == log_level
                    && config.webhook_url == webhook_url
                    && config.port == port
                    && config.ai_enabled == ai_enabled
            )
        }
        Err(_) => {
            // Valid configuration should not fail to load
            TestResult::failed()
        }
    }
}

/// **Feature: telegram-bot-template, Property 5: Configuration Validation Completeness**
/// For any invalid configuration input, the application should detect the invalidity 
/// and provide specific error messages without starting the bot
/// **Validates: Requirements 4.3**
fn property_configuration_validation_completeness(
    bot_token: String,
    log_level: String,
    webhook_url: Option<String>,
    port: Option<u16>,
) -> TestResult {
    // Create a configuration with potentially invalid values
    let config = Config {
        bot_token: bot_token.clone(),
        log_level: log_level.clone(),
        webhook_url: webhook_url.clone(),
        port,
        ai_enabled: false, // This field is always valid
        database_url: None,
    };

    let validation_result = config.validate();
    
    // Check if the configuration should be invalid
    let should_be_invalid = 
        // Empty bot token
        bot_token.is_empty() ||
        // Bot token without colon
        !bot_token.contains(':') ||
        // Invalid log level
        !["debug", "info", "warn", "error"].contains(&log_level.as_str()) ||
        // Invalid webhook URL format
        webhook_url.as_ref().map_or(false, |url| 
            !url.starts_with("http://") && !url.starts_with("https://")) ||
        // Invalid port range
        port.map_or(false, |p| p < 1024);

    match (should_be_invalid, validation_result) {
        (true, Err(_)) => TestResult::passed(), // Invalid config correctly rejected
        (false, Ok(_)) => TestResult::passed(), // Valid config correctly accepted
        (true, Ok(_)) => TestResult::failed(),  // Invalid config incorrectly accepted
        (false, Err(_)) => TestResult::failed(), // Valid config incorrectly rejected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_configuration_loading_reliability() {
        fn prop(bot_token: String, log_level: String, webhook_url: Option<String>, port: Option<u16>, ai_enabled: bool) -> TestResult {
            property_configuration_loading_reliability(bot_token, log_level, webhook_url, port, ai_enabled)
        }
        quickcheck(prop as fn(String, String, Option<String>, Option<u16>, bool) -> TestResult);
    }

    #[test]
    fn test_property_configuration_validation_completeness() {
        fn prop(bot_token: String, log_level: String, webhook_url: Option<String>, port: Option<u16>) -> TestResult {
            property_configuration_validation_completeness(bot_token, log_level, webhook_url, port)
        }
        quickcheck(prop as fn(String, String, Option<String>, Option<u16>) -> TestResult);
    }
}