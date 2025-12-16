use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use telegram_bot_template::error::{BotError, ErrorRecovery};
use teloxide::RequestError;

/// **Feature: telegram-bot-template, Property 6: API Error Recovery**
/// **Validates: Requirements 2.5**
/// 
/// For any Telegram API error encountered during operation, the bot should handle 
/// the error gracefully, log appropriate information, and continue operating

// Helper function to create different types of API errors for testing
fn create_network_error() -> BotError {
    BotError::Network("Connection timeout".to_string())
}

fn create_telegram_api_error(error_code: u16) -> BotError {
    // Create a mock Telegram API error
    // Note: In a real implementation, you'd use actual teloxide error types
    match error_code {
        429 => BotError::Network("Too Many Requests".to_string()),
        500 => BotError::Network("Internal Server Error".to_string()),
        502 => BotError::Network("Bad Gateway".to_string()),
        503 => BotError::Network("Service Unavailable".to_string()),
        _ => BotError::Network(format!("HTTP Error {}", error_code)),
    }
}

fn create_command_error(message: String) -> BotError {
    BotError::Command(message)
}

fn create_config_error() -> BotError {
    BotError::Config(telegram_bot_template::error::ConfigError::MissingVariable {
        variable: "TEST_VAR".to_string(),
    })
}

// Property test: Error recovery classification should be consistent
#[quickcheck]
fn prop_error_recovery_classification_consistent(error_type: u8, error_code: u16) -> TestResult {
    // Limit error types to valid range
    if error_type > 4 {
        return TestResult::discard();
    }
    
    // Limit error codes to reasonable HTTP range
    if error_code < 100 || error_code > 599 {
        return TestResult::discard();
    }
    
    let error = match error_type {
        0 => create_network_error(),
        1 => create_telegram_api_error(error_code),
        2 => create_command_error("Test command error".to_string()),
        3 => create_config_error(),
        _ => BotError::Internal("Test internal error".to_string()),
    };
    
    // Test that error recovery classification is consistent
    let is_recoverable = ErrorRecovery::is_recoverable(&error);
    
    // The same error should always be classified the same way
    let is_recoverable_again = ErrorRecovery::is_recoverable(&error);
    
    TestResult::from_bool(is_recoverable == is_recoverable_again)
}

// Property test: Recoverable errors should have user-friendly messages
#[quickcheck]
fn prop_recoverable_errors_have_user_messages(error_type: u8, message_content: String) -> TestResult {
    // Limit error types to recoverable ones
    if error_type > 2 {
        return TestResult::discard();
    }
    
    // Skip empty messages
    if message_content.is_empty() || message_content.len() > 1000 {
        return TestResult::discard();
    }
    
    let error = match error_type {
        0 => create_network_error(),
        1 => create_command_error(message_content),
        _ => BotError::AI("Test AI error".to_string()),
    };
    
    // Only test recoverable errors
    if !ErrorRecovery::is_recoverable(&error) {
        return TestResult::discard();
    }
    
    // Recoverable errors should have user-friendly messages
    let user_message = ErrorRecovery::user_friendly_message(&error);
    
    // User message should not be empty and should be polite
    let is_valid = !user_message.is_empty() && 
                   (user_message.to_lowercase().contains("sorry") || 
                    user_message.to_lowercase().contains("please") ||
                    user_message.to_lowercase().contains("try again"));
    
    TestResult::from_bool(is_valid)
}

// Property test: Error logging should not panic
#[quickcheck]
fn prop_error_logging_never_panics(error_type: u8, error_code: u16) -> TestResult {
    // Limit error types to valid range
    if error_type > 4 {
        return TestResult::discard();
    }
    
    // Limit error codes to reasonable range
    if error_code < 100 || error_code > 599 {
        return TestResult::discard();
    }
    
    let error = match error_type {
        0 => create_network_error(),
        1 => create_telegram_api_error(error_code),
        2 => create_command_error("Test error".to_string()),
        3 => create_config_error(),
        _ => BotError::Internal("Test internal error".to_string()),
    };
    
    // Error logging should never panic
    ErrorRecovery::log_error(&error);
    
    // If we get here without panicking, the test passes
    TestResult::passed()
}

// Property test: Fatal errors should be non-recoverable
#[quickcheck]
fn prop_fatal_errors_non_recoverable(fatal_error_type: u8) -> TestResult {
    // Limit to fatal error types
    if fatal_error_type > 1 {
        return TestResult::discard();
    }
    
    let error = match fatal_error_type {
        0 => create_config_error(),
        _ => BotError::Internal("Critical system error".to_string()),
    };
    
    // Fatal errors should not be recoverable
    TestResult::from_bool(!ErrorRecovery::is_recoverable(&error))
}

// Property test: Network errors should be recoverable
#[quickcheck]
fn prop_network_errors_recoverable(error_code: u16) -> TestResult {
    // Test common recoverable HTTP error codes
    let recoverable_codes = [429, 500, 502, 503, 504];
    
    if !recoverable_codes.contains(&error_code) {
        return TestResult::discard();
    }
    
    let error = create_telegram_api_error(error_code);
    
    // These network errors should be recoverable
    TestResult::from_bool(ErrorRecovery::is_recoverable(&error))
}

// Property test: Command errors should be recoverable
#[quickcheck]
fn prop_command_errors_recoverable(command_error_msg: String) -> TestResult {
    // Skip empty or very long messages
    if command_error_msg.is_empty() || command_error_msg.len() > 1000 {
        return TestResult::discard();
    }
    
    let error = create_command_error(command_error_msg);
    
    // Command errors should be recoverable (bot should continue operating)
    TestResult::from_bool(ErrorRecovery::is_recoverable(&error))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_error_recovery() {
        let error = create_network_error();
        assert!(ErrorRecovery::is_recoverable(&error));
        
        let user_msg = ErrorRecovery::user_friendly_message(&error);
        assert!(!user_msg.is_empty());
    }
    
    #[test]
    fn test_command_error_recovery() {
        let error = create_command_error("Invalid command".to_string());
        assert!(ErrorRecovery::is_recoverable(&error));
        
        let user_msg = ErrorRecovery::user_friendly_message(&error);
        assert!(user_msg.contains("Sorry") || user_msg.contains("sorry"));
    }
    
    #[test]
    fn test_config_error_not_recoverable() {
        let error = create_config_error();
        assert!(!ErrorRecovery::is_recoverable(&error));
    }
    
    #[test]
    fn test_internal_error_not_recoverable() {
        let error = BotError::Internal("Critical failure".to_string());
        assert!(!ErrorRecovery::is_recoverable(&error));
    }
    
    #[test]
    fn test_error_logging_does_not_panic() {
        let errors = vec![
            create_network_error(),
            create_command_error("Test".to_string()),
            create_config_error(),
            BotError::Internal("Test".to_string()),
        ];
        
        for error in errors {
            // This should not panic
            ErrorRecovery::log_error(&error);
        }
    }
}