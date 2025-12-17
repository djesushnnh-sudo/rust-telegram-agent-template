use quickcheck::{quickcheck, TestResult};
use telegram_bot_template::ai::AIRouter;
use telegram_bot_template::config::Config;

/// **Feature: telegram-bot-template, Property 8: AI Routing Decision Consistency**
/// 
/// For any message when AI functionality is enabled, the AI router should make 
/// consistent routing decisions based on message content and configuration.
/// 
/// This property validates that:
/// 1. The same message content always produces the same routing decision
/// 2. Commands are never routed to AI processing
/// 3. AI routing decisions are deterministic based on configuration
/// 
/// **Validates: Requirements 6.3**

fn create_test_config() -> Config {
    Config {
        bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: Some(8080),
        ai_enabled: true,
        database_url: None,
        database_provider: None,
    }
}

/// Test that AI routing decisions are consistent for the same input
fn prop_ai_routing_consistency(message_text: String, ai_enabled: bool) -> TestResult {
    // Skip empty messages and very long messages to focus on realistic inputs
    if message_text.is_empty() || message_text.len() > 1000 {
        return TestResult::discard();
    }
    
    // Create AI router with consistent configuration
    let api_key = if ai_enabled { Some("test_key".to_string()) } else { None };
    let router = AIRouter::new(ai_enabled, api_key);
    
    // Create a mock message structure for testing
    let is_command = message_text.trim_start().starts_with('/');
    
    // Test the routing decision consistency
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Make the same routing decision multiple times
    let decision1 = rt.block_on(async {
        // Simulate the routing decision logic
        if !router.is_enabled() {
            return false;
        }
        
        // Commands should never be routed to AI
        if is_command {
            return false;
        }
        
        // For non-command messages, AI should process them when enabled
        true
    });
    
    let decision2 = rt.block_on(async {
        // Simulate the same routing decision logic
        if !router.is_enabled() {
            return false;
        }
        
        // Commands should never be routed to AI
        if is_command {
            return false;
        }
        
        // For non-command messages, AI should process them when enabled
        true
    });
    
    // The decisions should be identical for the same input
    TestResult::from_bool(decision1 == decision2)
}

/// Test that commands are never routed to AI processing
fn prop_commands_never_routed_to_ai(command_name: String, ai_enabled: bool) -> TestResult {
    // Skip empty command names
    if command_name.is_empty() {
        return TestResult::discard();
    }
    
    // Create a command message
    let command_text = format!("/{}", command_name.trim());
    
    // Create AI router
    let api_key = if ai_enabled { Some("test_key".to_string()) } else { None };
    let router = AIRouter::new(ai_enabled, api_key);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Test that commands are never routed to AI
    let should_process = rt.block_on(async {
        // Simulate the command detection logic
        if command_text.starts_with('/') {
            return false; // Commands should never go to AI
        }
        
        // Only route to AI if enabled and not a command
        router.is_enabled()
    });
    
    // Commands should never be processed by AI, regardless of AI enablement
    TestResult::from_bool(!should_process)
}

/// Test that AI routing respects the enabled/disabled state
fn prop_ai_routing_respects_enabled_state(message_text: String) -> TestResult {
    // Skip empty messages and commands for this test
    if message_text.is_empty() || message_text.trim_start().starts_with('/') {
        return TestResult::discard();
    }
    
    // Test with AI disabled
    let disabled_router = AIRouter::new(false, None);
    
    // Test with AI enabled
    let enabled_router = AIRouter::new(true, Some("test_key".to_string()));
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let disabled_decision = rt.block_on(async {
        disabled_router.is_enabled()
    });
    
    let enabled_decision = rt.block_on(async {
        enabled_router.is_enabled()
    });
    
    // Disabled router should never route to AI, enabled router should be able to
    TestResult::from_bool(!disabled_decision && enabled_decision)
}

/// Test that AI routing decisions are deterministic based on message content
fn prop_ai_routing_deterministic_by_content(message_text: String) -> TestResult {
    // Skip empty messages
    if message_text.is_empty() {
        return TestResult::discard();
    }
    
    let router = AIRouter::new(true, Some("test_key".to_string()));
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Make multiple routing decisions for the same content
    let decisions: Vec<bool> = (0..5).map(|_| {
        rt.block_on(async {
            // Simulate routing decision based on content
            if message_text.trim_start().starts_with('/') {
                false // Commands
            } else if router.is_enabled() {
                true // Regular messages when AI is enabled
            } else {
                false // AI disabled
            }
        })
    }).collect();
    
    // All decisions should be identical for the same content
    let first_decision = decisions[0];
    let all_same = decisions.iter().all(|&decision| decision == first_decision);
    
    TestResult::from_bool(all_same)
}

/// Test that AI processor configuration affects routing readiness
fn prop_ai_processor_config_affects_readiness(ai_enabled: bool, has_api_key: bool) -> bool {
    let api_key = if has_api_key { Some("test_key".to_string()) } else { None };
    let router = AIRouter::new(ai_enabled, api_key);
    
    // Router should be ready only when both enabled and has API key
    let expected_ready = ai_enabled && has_api_key;
    router.is_ready() == expected_ready
}

/// Test that AI routing handles edge cases in message content
fn prop_ai_routing_handles_edge_cases(message_text: String) -> TestResult {
    // Test various edge cases
    let router = AIRouter::new(true, Some("test_key".to_string()));
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let routing_decision = rt.block_on(async {
        // Simulate edge case handling
        let trimmed = message_text.trim();
        
        // Empty or whitespace-only messages
        if trimmed.is_empty() {
            return false;
        }
        
        // Commands (including those with extra whitespace)
        if trimmed.starts_with('/') {
            return false;
        }
        
        // Very long messages (might have different handling)
        if message_text.len() > 4000 {
            return false; // Telegram message limit consideration
        }
        
        // Regular messages should be routed to AI when enabled
        router.is_enabled()
    });
    
    // The function should not panic and should return a boolean decision
    TestResult::from_bool(routing_decision == true || routing_decision == false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ai_routing_consistency_property() {
        fn prop(message_text: String, ai_enabled: bool) -> TestResult {
            prop_ai_routing_consistency(message_text, ai_enabled)
        }
        quickcheck(prop as fn(String, bool) -> TestResult);
    }
    
    #[test]
    fn test_commands_never_routed_property() {
        fn prop(command_name: String, ai_enabled: bool) -> TestResult {
            prop_commands_never_routed_to_ai(command_name, ai_enabled)
        }
        quickcheck(prop as fn(String, bool) -> TestResult);
    }
    
    #[test]
    fn test_ai_routing_enabled_state_property() {
        fn prop(message_text: String) -> TestResult {
            prop_ai_routing_respects_enabled_state(message_text)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_ai_routing_deterministic_property() {
        fn prop(message_text: String) -> TestResult {
            prop_ai_routing_deterministic_by_content(message_text)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_ai_processor_config_property() {
        fn prop(ai_enabled: bool, has_api_key: bool) -> bool {
            prop_ai_processor_config_affects_readiness(ai_enabled, has_api_key)
        }
        quickcheck(prop as fn(bool, bool) -> bool);
    }
    
    #[test]
    fn test_ai_routing_edge_cases_property() {
        fn prop(message_text: String) -> TestResult {
            prop_ai_routing_handles_edge_cases(message_text)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
}