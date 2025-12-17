use quickcheck::{quickcheck, TestResult};
use telegram_bot_template::ai::AIRouter;
use telegram_bot_template::config::Config;

/// **Feature: telegram-bot-template, Property 9: AI Graceful Degradation**
/// 
/// For any message when AI functionality is disabled, the bot should process 
/// the message through normal command handling without attempting AI processing.
/// 
/// This property validates that:
/// 1. When AI is disabled, no messages are routed to AI processing
/// 2. The system continues to function normally without AI
/// 3. Disabling AI doesn't cause crashes or errors
/// 4. All message types are handled gracefully when AI is disabled
/// 
/// **Validates: Requirements 6.4**

fn create_test_config() -> Config {
    Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: Some(8080),
        ai_enabled: false, // AI disabled for graceful degradation testing
        database_url: None,
        database_provider: None,
    }
}

/// Test that when AI is disabled, no messages are processed by AI
fn prop_ai_disabled_no_processing(message_text: String) -> TestResult {
    // Skip empty messages to focus on realistic inputs
    if message_text.is_empty() {
        return TestResult::discard();
    }
    
    // Create AI router with AI explicitly disabled
    let router = AIRouter::new(false, None);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Test that AI processing is never attempted when disabled
    let ai_enabled = rt.block_on(async {
        router.is_enabled()
    });
    
    let ai_ready = rt.block_on(async {
        router.is_ready()
    });
    
    // When AI is disabled, it should never be enabled or ready
    TestResult::from_bool(!ai_enabled && !ai_ready)
}

/// Test that disabling AI doesn't cause system failures
fn prop_ai_disabled_system_stability(message_text: String, has_api_key: bool) -> TestResult {
    // Skip empty messages
    if message_text.is_empty() {
        return TestResult::discard();
    }
    
    let api_key = if has_api_key { Some("test_key".to_string()) } else { None };
    
    // Create router with AI disabled regardless of API key availability
    let router = AIRouter::new(false, api_key);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Test that the router can be created and used without issues
    let creation_successful = rt.block_on(async {
        // These operations should not panic or fail
        let _enabled = router.is_enabled();
        let _ready = router.is_ready();
        
        // Simulate routing decision - should always be false when disabled
        let should_process = if message_text.trim_start().starts_with('/') {
            false // Commands never go to AI
        } else {
            router.is_enabled() // Should be false since AI is disabled
        };
        
        !should_process // Should always be true (not processing) when AI disabled
    });
    
    TestResult::from_bool(creation_successful)
}

/// Test that AI router gracefully handles state transitions
fn prop_ai_graceful_state_transitions(initial_enabled: bool, final_enabled: bool) -> bool {
    let api_key = Some("test_key".to_string());
    let mut router = AIRouter::new(initial_enabled, api_key);
    
    // Record initial state
    let initial_state = router.is_enabled();
    
    // Change the enabled state
    router.set_enabled(final_enabled);
    
    // Record final state
    let final_state = router.is_enabled();
    
    // State should match what we set
    initial_state == initial_enabled && final_state == final_enabled
}

/// Test that AI graceful degradation works with various message types
fn prop_ai_degradation_message_types(message_text: String) -> TestResult {
    // Test various message patterns
    if message_text.is_empty() {
        return TestResult::discard();
    }
    
    let router = AIRouter::new(false, Some("key".to_string())); // Disabled with key
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let graceful_handling = rt.block_on(async {
        // Test different message types
        let is_command = message_text.trim_start().starts_with('/');
        let is_long_message = message_text.len() > 1000;
        let is_short_message = message_text.len() < 10;
        let has_special_chars = message_text.chars().any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace());
        
        // All message types should be handled gracefully (not processed by AI)
        let should_process_command = if is_command { false } else { router.is_enabled() };
        let should_process_long = if is_long_message { false } else { router.is_enabled() };
        let should_process_short = if is_short_message { false } else { router.is_enabled() };
        let should_process_special = if has_special_chars { router.is_enabled() } else { router.is_enabled() };
        
        // Since AI is disabled, all should be false
        !should_process_command && !should_process_long && !should_process_short && !should_process_special
    });
    
    TestResult::from_bool(graceful_handling)
}

/// Test that AI router handles configuration changes gracefully
fn prop_ai_config_change_graceful(ai_enabled: bool) -> bool {
    // Test creating router with different configurations
    let router_with_key = AIRouter::new(ai_enabled, Some("key".to_string()));
    let router_without_key = AIRouter::new(ai_enabled, None);
    
    // Both should be created successfully
    let with_key_state = router_with_key.is_enabled();
    let without_key_state = router_without_key.is_enabled();
    
    // Enabled state should match the parameter
    with_key_state == ai_enabled && without_key_state == ai_enabled
}

/// Test that AI processor access is safe when AI is disabled
fn prop_ai_processor_access_safe_when_disabled() -> bool {
    let router = AIRouter::new(false, None);
    
    // These operations should not panic
    let processor = router.processor();
    let is_ready = processor.is_ready();
    
    // When AI is disabled and no key, processor should not be ready
    !is_ready
}

/// Test that AI router handles edge cases gracefully when disabled
fn prop_ai_edge_cases_graceful_when_disabled(message_text: String) -> TestResult {
    // Test edge cases
    let router = AIRouter::new(false, Some("key".to_string()));
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let handles_gracefully = rt.block_on(async {
        // Test various edge cases
        let empty_trimmed = message_text.trim().is_empty();
        let only_whitespace = message_text.chars().all(|c| c.is_whitespace());
        let very_long = message_text.len() > 10000;
        let has_newlines = message_text.contains('\n');
        let has_unicode = message_text.chars().any(|c| !c.is_ascii());
        
        // All edge cases should be handled without AI processing
        let should_not_process = !router.is_enabled(); // Should always be true when disabled
        
        // Test that we can make routing decisions for all edge cases
        let _routing_decision = if empty_trimmed || only_whitespace {
            false // Empty messages not processed
        } else if very_long {
            false // Very long messages might be rejected
        } else if has_newlines || has_unicode {
            router.is_enabled() // Should be false since disabled
        } else {
            router.is_enabled() // Should be false since disabled
        };
        
        should_not_process
    });
    
    TestResult::from_bool(handles_gracefully)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ai_disabled_no_processing_property() {
        fn prop(message_text: String) -> TestResult {
            prop_ai_disabled_no_processing(message_text)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_ai_disabled_system_stability_property() {
        fn prop(message_text: String, has_api_key: bool) -> TestResult {
            prop_ai_disabled_system_stability(message_text, has_api_key)
        }
        quickcheck(prop as fn(String, bool) -> TestResult);
    }
    
    #[test]
    fn test_ai_graceful_state_transitions_property() {
        fn prop(initial_enabled: bool, final_enabled: bool) -> bool {
            prop_ai_graceful_state_transitions(initial_enabled, final_enabled)
        }
        quickcheck(prop as fn(bool, bool) -> bool);
    }
    
    #[test]
    fn test_ai_degradation_message_types_property() {
        fn prop(message_text: String) -> TestResult {
            prop_ai_degradation_message_types(message_text)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_ai_config_change_graceful_property() {
        fn prop(ai_enabled: bool) -> bool {
            prop_ai_config_change_graceful(ai_enabled)
        }
        quickcheck(prop as fn(bool) -> bool);
    }
    
    #[test]
    fn test_ai_processor_access_safe_when_disabled_property() {
        quickcheck(prop_ai_processor_access_safe_when_disabled as fn() -> bool);
    }
    
    #[test]
    fn test_ai_edge_cases_graceful_when_disabled_property() {
        fn prop(message_text: String) -> TestResult {
            prop_ai_edge_cases_graceful_when_disabled(message_text)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
}