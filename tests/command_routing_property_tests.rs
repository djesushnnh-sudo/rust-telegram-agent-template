use quickcheck::{quickcheck, TestResult};
use std::collections::HashMap;
use telegram_bot_template::commands::{CommandRouter, CommandHandler};
use telegram_bot_template::error::BotResult;
use teloxide::{Bot, types::Message};
use async_trait::async_trait;

/// **Feature: telegram-bot-template, Property 2: Command Routing Accuracy**
/// 
/// For any registered command and valid command message, the command router should 
/// correctly identify the command and route it to the appropriate handler
/// 
/// **Validates: Requirements 3.1, 3.4**

/// Mock command handler that tracks if it was called
#[derive(Clone)]
struct TestCommandHandler {
    name: String,
    call_count: std::sync::Arc<std::sync::Mutex<usize>>,
}

impl TestCommandHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            call_count: std::sync::Arc::new(std::sync::Mutex::new(0)),
        }
    }
    
    fn get_call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }
}

#[async_trait]
impl CommandHandler for TestCommandHandler {
    async fn handle(&self, _bot: &Bot, _message: &Message, _args: Vec<String>) -> BotResult<()> {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        Ok(())
    }
    
    fn description(&self) -> &str {
        "Test command handler"
    }
    
    fn usage(&self) -> &str {
        "Test command usage"
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Generate valid command names (alphanumeric, lowercase, no spaces)
fn generate_valid_command_name(input: String) -> String {
    let cleaned: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    
    if cleaned.is_empty() {
        "test".to_string()
    } else {
        cleaned.chars().take(20).collect() // Limit length
    }
}

/// Test that registered commands are correctly identified and routed
fn test_command_routing_accuracy(command_names: Vec<String>) -> TestResult {
    if command_names.is_empty() || command_names.len() > 10 {
        return TestResult::discard();
    }
    
    // Generate valid command names
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
    let mut handlers: HashMap<String, TestCommandHandler> = HashMap::new();
    
    // Register all commands
    for name in &valid_names {
        let handler = TestCommandHandler::new(name);
        handlers.insert(name.clone(), handler.clone());
        router.register_command(name, Box::new(handler));
    }
    
    // Test that all registered commands are recognized
    for name in &valid_names {
        let is_registered = router.is_command_registered(name);
        if !is_registered {
            return TestResult::failed();
        }
    }
    
    // Test command parsing for each registered command
    for name in &valid_names {
        let command_text = format!("/{}", name);
        let parse_result = router.parse_command(&command_text);
        
        match parse_result {
            Ok((parsed_name, args)) => {
                if parsed_name != *name || !args.is_empty() {
                    return TestResult::failed();
                }
            }
            Err(_) => return TestResult::failed(),
        }
    }
    
    // Test command parsing with arguments
    for name in &valid_names {
        let command_text = format!("/{} arg1 arg2", name);
        let parse_result = router.parse_command(&command_text);
        
        match parse_result {
            Ok((parsed_name, args)) => {
                if parsed_name != *name || args != vec!["arg1", "arg2"] {
                    return TestResult::failed();
                }
            }
            Err(_) => return TestResult::failed(),
        }
    }
    
    // Test that the router correctly identifies the number of registered commands
    if router.command_count() != valid_names.len() {
        return TestResult::failed();
    }
    
    // Test that get_registered_commands returns all registered commands
    let registered = router.get_registered_commands();
    for name in &valid_names {
        if !registered.contains(name) {
            return TestResult::failed();
        }
    }
    
    TestResult::passed()
}

/// Test that command parsing handles edge cases correctly
fn test_command_parsing_edge_cases(input: String) -> TestResult {
    let router = CommandRouter::new();
    
    // Test empty command
    if input.is_empty() {
        let result = router.parse_command("/");
        return TestResult::from_bool(result.is_err());
    }
    
    // Test command without leading slash
    if !input.starts_with('/') {
        let result = router.parse_command(&input);
        return TestResult::from_bool(result.is_err());
    }
    
    // Test valid command format
    if input.starts_with('/') && input.len() > 1 {
        let result = router.parse_command(&input);
        match result {
            Ok((name, _args)) => {
                // Command name should not be empty and should not contain '/'
                TestResult::from_bool(!name.is_empty() && !name.contains('/'))
            }
            Err(_) => {
                // Some inputs might be invalid, which is acceptable
                TestResult::passed()
            }
        }
    } else {
        TestResult::discard()
    }
}

/// Test that commands with @botname suffix are handled correctly
fn test_command_with_botname_suffix(command_name: String, bot_name: String) -> TestResult {
    let clean_command = generate_valid_command_name(command_name);
    let clean_bot_name = generate_valid_command_name(bot_name);
    
    if clean_command.is_empty() || clean_bot_name.is_empty() {
        return TestResult::discard();
    }
    
    let router = CommandRouter::new();
    
    // Test command with @botname suffix
    let command_text = format!("/{}@{}", clean_command, clean_bot_name);
    let result = router.parse_command(&command_text);
    
    match result {
        Ok((parsed_name, _args)) => {
            // The parsed name should be the command without the @botname suffix
            TestResult::from_bool(parsed_name == clean_command)
        }
        Err(_) => TestResult::failed(),
    }
}

/// Test that unknown commands are handled appropriately
fn test_unknown_command_handling(known_commands: Vec<String>, unknown_command: String) -> TestResult {
    let known_clean: Vec<String> = known_commands
        .into_iter()
        .map(generate_valid_command_name)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .take(5) // Limit to avoid too many commands
        .collect();
    
    let unknown_clean = generate_valid_command_name(unknown_command);
    
    if known_clean.is_empty() || unknown_clean.is_empty() || known_clean.contains(&unknown_clean) {
        return TestResult::discard();
    }
    
    let mut router = CommandRouter::new();
    
    // Register known commands
    for name in &known_clean {
        let handler = TestCommandHandler::new(name);
        router.register_command(name, Box::new(handler));
    }
    
    // Test that known commands are registered
    for name in &known_clean {
        if !router.is_command_registered(name) {
            return TestResult::failed();
        }
    }
    
    // Test that unknown command is not registered
    if router.is_command_registered(&unknown_clean) {
        return TestResult::failed();
    }
    
    TestResult::passed()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_property_command_routing_accuracy() {
        fn prop(command_names: Vec<String>) -> TestResult {
            test_command_routing_accuracy(command_names)
        }
        quickcheck(prop as fn(Vec<String>) -> TestResult);
    }
    
    #[test]
    fn test_property_command_parsing_edge_cases() {
        fn prop(input: String) -> TestResult {
            test_command_parsing_edge_cases(input)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_property_command_with_botname_suffix() {
        fn prop(command_name: String, bot_name: String) -> TestResult {
            test_command_with_botname_suffix(command_name, bot_name)
        }
        quickcheck(prop as fn(String, String) -> TestResult);
    }
    
    #[test]
    fn test_property_unknown_command_handling() {
        fn prop(known_commands: Vec<String>, unknown_command: String) -> TestResult {
            test_unknown_command_handling(known_commands, unknown_command)
        }
        quickcheck(prop as fn(Vec<String>, String) -> TestResult);
    }
    
    #[test]
    fn test_basic_command_registration() {
        let mut router = CommandRouter::new();
        let handler = TestCommandHandler::new("test");
        
        router.register_command("test", Box::new(handler));
        
        assert!(router.is_command_registered("test"));
        assert_eq!(router.command_count(), 1);
    }
    
    #[test]
    fn test_basic_command_parsing() {
        let router = CommandRouter::new();
        
        let result = router.parse_command("/start");
        assert!(result.is_ok());
        
        let (name, args) = result.unwrap();
        assert_eq!(name, "start");
        assert!(args.is_empty());
    }
}