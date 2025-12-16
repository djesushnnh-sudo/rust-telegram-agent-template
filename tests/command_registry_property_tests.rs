use quickcheck::{quickcheck, TestResult};
use telegram_bot_template::commands::{CommandRouter, CommandHandler};
use telegram_bot_template::error::BotResult;
use teloxide::{Bot, types::Message};
use async_trait::async_trait;

/// **Feature: telegram-bot-template, Property 3: Command Registry Consistency**
/// 
/// For any set of commands registered with the command handler, all registered commands 
/// should remain accessible and executable throughout the bot's lifecycle
/// 
/// **Validates: Requirements 3.5**

/// Mock command handler for testing registry consistency
#[derive(Clone)]
struct RegistryTestHandler {
    name: String,
    description: String,
}

impl RegistryTestHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: format!("Handler for {}", name),
        }
    }
}

#[async_trait]
impl CommandHandler for RegistryTestHandler {
    async fn handle(&self, _bot: &Bot, _message: &Message, _args: Vec<String>) -> BotResult<()> {
        Ok(())
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn usage(&self) -> &str {
        "Test registry handler usage"
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Generate valid command names (ASCII alphanumeric only, lowercase, no spaces)
fn generate_valid_command_name(input: String) -> String {
    let cleaned: String = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    
    if cleaned.is_empty() {
        "test".to_string()
    } else {
        cleaned.chars().take(20).collect() // Limit length
    }
}

/// Test that command registry maintains consistency across operations
fn test_command_registry_consistency(command_names: Vec<String>) -> TestResult {
    if command_names.is_empty() || command_names.len() > 15 {
        return TestResult::discard();
    }
    
    // Generate valid, unique command names
    let valid_names: Vec<String> = command_names
        .into_iter()
        .map(generate_valid_command_name)
        .collect::<std::collections::HashSet<_>>() // Remove duplicates
        .into_iter()
        .collect();
    
    if valid_names.is_empty() {
        return TestResult::discard();
    }
    
    let mut router = CommandRouter::new();
    
    // Initially, no commands should be registered
    if router.command_count() != 0 {
        return TestResult::failed();
    }
    
    if !router.get_registered_commands().is_empty() {
        return TestResult::failed();
    }
    
    // Register all commands one by one and verify consistency at each step
    for (index, name) in valid_names.iter().enumerate() {
        let handler = RegistryTestHandler::new(name);
        router.register_command(name, Box::new(handler));
        
        // After registering, the command should be registered
        if !router.is_command_registered(name) {
            return TestResult::failed();
        }
        
        // Command count should match the number of registered commands
        if router.command_count() != index + 1 {
            return TestResult::failed();
        }
        
        // All previously registered commands should still be registered
        for prev_name in valid_names.iter().take(index + 1) {
            if !router.is_command_registered(prev_name) {
                return TestResult::failed();
            }
        }
        
        // get_registered_commands should return all registered commands so far
        let registered = router.get_registered_commands();
        if registered.len() != index + 1 {
            return TestResult::failed();
        }
        
        for prev_name in valid_names.iter().take(index + 1) {
            if !registered.contains(prev_name) {
                return TestResult::failed();
            }
        }
    }
    
    // Final verification: all commands should be registered
    for name in &valid_names {
        if !router.is_command_registered(name) {
            return TestResult::failed();
        }
    }
    
    // Final count should match total number of unique commands
    if router.command_count() != valid_names.len() {
        return TestResult::failed();
    }
    
    // get_registered_commands should return exactly the registered commands
    let final_registered = router.get_registered_commands();
    if final_registered.len() != valid_names.len() {
        return TestResult::failed();
    }
    
    for name in &valid_names {
        if !final_registered.contains(name) {
            return TestResult::failed();
        }
    }
    
    TestResult::passed()
}

/// Test that command registration is case-insensitive but preserves original case
fn test_command_registry_case_handling(command_name: String) -> TestResult {
    let clean_name = generate_valid_command_name(command_name);
    
    if clean_name.is_empty() {
        return TestResult::discard();
    }
    
    let mut router = CommandRouter::new();
    let handler = RegistryTestHandler::new(&clean_name);
    
    // Register command with original case
    router.register_command(&clean_name, Box::new(handler));
    
    // Should be registered under the lowercase version
    if !router.is_command_registered(&clean_name.to_lowercase()) {
        return TestResult::failed();
    }
    
    // Should also be found with uppercase version (case insensitive lookup)
    if !router.is_command_registered(&clean_name.to_uppercase()) {
        return TestResult::failed();
    }
    
    // Should be found with mixed case (only test if all characters are ASCII)
    if clean_name.chars().all(|c| c.is_ascii()) {
        let mixed_case: String = clean_name
            .chars()
            .enumerate()
            .map(|(i, c)| if i % 2 == 0 { c.to_uppercase().collect::<String>() } else { c.to_lowercase().collect::<String>() })
            .collect();
        
        if !router.is_command_registered(&mixed_case) {
            return TestResult::failed();
        }
    }
    
    TestResult::passed()
}

/// Test that duplicate command registration overwrites previous registration
fn test_command_registry_overwrite_behavior(command_name: String) -> TestResult {
    let clean_name = generate_valid_command_name(command_name);
    
    if clean_name.is_empty() {
        return TestResult::discard();
    }
    
    let mut router = CommandRouter::new();
    
    // Register first handler
    let handler1 = RegistryTestHandler::new(&clean_name);
    router.register_command(&clean_name, Box::new(handler1));
    
    // Should be registered
    if !router.is_command_registered(&clean_name) {
        return TestResult::failed();
    }
    
    // Count should be 1
    if router.command_count() != 1 {
        return TestResult::failed();
    }
    
    // Register second handler with same name (should overwrite)
    let handler2 = RegistryTestHandler::new(&clean_name);
    router.register_command(&clean_name, Box::new(handler2));
    
    // Should still be registered
    if !router.is_command_registered(&clean_name) {
        return TestResult::failed();
    }
    
    // Count should still be 1 (not 2, because it was overwritten)
    if router.command_count() != 1 {
        return TestResult::failed();
    }
    
    // get_registered_commands should still return only one command
    let registered = router.get_registered_commands();
    if registered.len() != 1 || !registered.contains(&clean_name) {
        return TestResult::failed();
    }
    
    TestResult::passed()
}

/// Test that empty or invalid command names are handled appropriately
fn test_command_registry_invalid_names(command_names: Vec<String>) -> TestResult {
    if command_names.len() > 10 {
        return TestResult::discard();
    }
    
    let mut router = CommandRouter::new();
    let mut valid_count = 0;
    
    for name in command_names {
        let clean_name = generate_valid_command_name(name);
        
        // Only register if we got a valid name
        if !clean_name.is_empty() && clean_name != "test" {
            let handler = RegistryTestHandler::new(&clean_name);
            router.register_command(&clean_name, Box::new(handler));
            valid_count += 1;
            
            // Should be registered
            if !router.is_command_registered(&clean_name) {
                return TestResult::failed();
            }
        }
    }
    
    // Command count should match the number of valid commands registered
    if router.command_count() != valid_count {
        return TestResult::failed();
    }
    
    TestResult::passed()
}

/// Test registry behavior with a large number of commands
fn test_command_registry_scalability(base_name: String) -> TestResult {
    let clean_base = generate_valid_command_name(base_name);
    
    if clean_base.is_empty() {
        return TestResult::discard();
    }
    
    let mut router = CommandRouter::new();
    let command_count = 50; // Test with 50 commands
    
    // Register many commands
    for i in 0..command_count {
        let command_name = format!("{}{}", clean_base, i);
        let handler = RegistryTestHandler::new(&command_name);
        router.register_command(&command_name, Box::new(handler));
    }
    
    // Verify all commands are registered
    for i in 0..command_count {
        let command_name = format!("{}{}", clean_base, i);
        if !router.is_command_registered(&command_name) {
            return TestResult::failed();
        }
    }
    
    // Verify total count
    if router.command_count() != command_count {
        return TestResult::failed();
    }
    
    // Verify get_registered_commands returns all commands
    let registered = router.get_registered_commands();
    if registered.len() != command_count {
        return TestResult::failed();
    }
    
    for i in 0..command_count {
        let command_name = format!("{}{}", clean_base, i);
        if !registered.contains(&command_name) {
            return TestResult::failed();
        }
    }
    
    TestResult::passed()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_property_command_registry_consistency() {
        fn prop(command_names: Vec<String>) -> TestResult {
            test_command_registry_consistency(command_names)
        }
        quickcheck(prop as fn(Vec<String>) -> TestResult);
    }
    
    #[test]
    fn test_property_command_registry_case_handling() {
        fn prop(command_name: String) -> TestResult {
            test_command_registry_case_handling(command_name)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_property_command_registry_overwrite_behavior() {
        fn prop(command_name: String) -> TestResult {
            test_command_registry_overwrite_behavior(command_name)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_property_command_registry_invalid_names() {
        fn prop(command_names: Vec<String>) -> TestResult {
            test_command_registry_invalid_names(command_names)
        }
        quickcheck(prop as fn(Vec<String>) -> TestResult);
    }
    
    #[test]
    fn test_property_command_registry_scalability() {
        fn prop(base_name: String) -> TestResult {
            test_command_registry_scalability(base_name)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_basic_registry_operations() {
        let mut router = CommandRouter::new();
        
        // Initially empty
        assert_eq!(router.command_count(), 0);
        assert!(router.get_registered_commands().is_empty());
        assert!(!router.is_command_registered("test"));
        
        // Register a command
        let handler = RegistryTestHandler::new("test");
        router.register_command("test", Box::new(handler));
        
        // Should be registered
        assert_eq!(router.command_count(), 1);
        assert!(router.is_command_registered("test"));
        assert!(router.get_registered_commands().contains(&"test".to_string()));
    }
    
    #[test]
    fn test_case_insensitive_lookup() {
        let mut router = CommandRouter::new();
        let handler = RegistryTestHandler::new("TestCommand");
        
        router.register_command("TestCommand", Box::new(handler));
        
        // Should find with different cases
        assert!(router.is_command_registered("testcommand"));
        assert!(router.is_command_registered("TESTCOMMAND"));
        assert!(router.is_command_registered("TestCommand"));
    }
}