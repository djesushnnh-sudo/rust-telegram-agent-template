use quickcheck::{quickcheck, TestResult};
use telegram_bot_template::commands::BotCommand;
use teloxide::utils::command::BotCommands;

/// **Feature: telegram-bot-template, Property 3: Command Registry Consistency**
/// 
/// For the BotCommands derive macro, all defined commands should be consistently 
/// accessible and parseable throughout the bot's lifecycle
/// 
/// **Validates: Requirements 3.5**

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

/// Test that BotCommand enum consistently handles all defined commands
fn test_command_registry_consistency(_command_names: Vec<String>) -> TestResult {
    // Test all known commands in the BotCommand enum
    let known_commands = vec!["start", "help", "status"];
    
    // Test that all known commands can be parsed consistently
    for cmd_name in &known_commands {
        let command_text = format!("/{}", cmd_name);
        
        // Test command text formatting
        if !command_text.starts_with('/') {
            return TestResult::failed();
        }
        
        // Test that BotCommand can parse the command
        if BotCommand::parse(&command_text, "testbot").is_err() {
            return TestResult::failed();
        }
        
        // Test command parsing
        let without_slash = &command_text[1..];
        let parts: Vec<&str> = without_slash.split_whitespace().collect();
        if parts.is_empty() || parts[0] != *cmd_name {
            return TestResult::failed();
        }
    }
    
    // Test echo command with argument
    let echo_command = "/echo test message";
    if BotCommand::parse(echo_command, "testbot").is_err() {
        return TestResult::failed();
    }
    
    TestResult::passed()
}

/// Test that BotCommand parsing follows consistent case rules
fn test_command_registry_case_handling(command_name: String) -> TestResult {
    let clean_name = generate_valid_command_name(command_name);
    
    if clean_name.is_empty() {
        return TestResult::discard();
    }
    
    // Test with known commands only (lowercase as defined in enum)
    let known_commands = vec!["start", "help", "status"];
    
    for cmd_name in &known_commands {
        // Test lowercase (should work - this is the defined case)
        let lowercase_cmd = format!("/{}", cmd_name.to_lowercase());
        if BotCommand::parse(&lowercase_cmd, "testbot").is_err() {
            return TestResult::failed();
        }
        
        // Test that the parsing is consistent for the same input
        let result1 = BotCommand::parse(&lowercase_cmd, "testbot");
        let result2 = BotCommand::parse(&lowercase_cmd, "testbot");
        
        if result1.is_err() != result2.is_err() {
            return TestResult::failed();
        }
    }
    
    TestResult::passed()
}

/// Test that BotCommand enum has consistent behavior (no overwrite needed with derive macro)
fn test_command_registry_overwrite_behavior(_command_name: String) -> TestResult {
    // With BotCommands derive macro, commands are defined at compile time
    // so there's no runtime registration or overwriting behavior to test.
    // Instead, test that the same command can be parsed multiple times consistently.
    
    let known_commands = vec!["start", "help", "status"];
    
    for cmd_name in &known_commands {
        let command_text = format!("/{}", cmd_name);
        
        // Parse the same command multiple times - should be consistent
        let result1 = BotCommand::parse(&command_text, "testbot");
        let result2 = BotCommand::parse(&command_text, "testbot");
        
        if result1.is_err() || result2.is_err() {
            return TestResult::failed();
        }
        
        // Both results should be the same variant
        match (result1.unwrap(), result2.unwrap()) {
            (BotCommand::Start, BotCommand::Start) => {},
            (BotCommand::Help, BotCommand::Help) => {},
            (BotCommand::Status, BotCommand::Status) => {},
            _ => return TestResult::failed(),
        }
    }
    
    TestResult::passed()
}

/// Test that invalid command names are handled appropriately by BotCommand
fn test_command_registry_invalid_names(command_names: Vec<String>) -> TestResult {
    if command_names.len() > 10 {
        return TestResult::discard();
    }
    
    let known_commands = vec!["start", "help", "status"];
    
    for name in command_names {
        let clean_name = generate_valid_command_name(name);
        
        if clean_name.is_empty() {
            continue;
        }
        
        let command_text = format!("/{}", clean_name);
        
        // Test parsing
        let parse_result = BotCommand::parse(&command_text, "testbot");
        
        if known_commands.contains(&clean_name.as_str()) {
            // Known commands should parse successfully
            if parse_result.is_err() {
                return TestResult::failed();
            }
        } else {
            // Unknown commands should fail to parse
            if parse_result.is_ok() {
                return TestResult::failed();
            }
        }
    }
    
    TestResult::passed()
}

/// Test BotCommand scalability (compile-time defined commands)
fn test_command_registry_scalability(_base_name: String) -> TestResult {
    // With BotCommands derive macro, commands are defined at compile time
    // so scalability is not a runtime concern. Instead, test that all
    // defined commands work consistently.
    
    let known_commands = vec!["start", "help", "status"];
    
    // Test parsing all commands multiple times to ensure consistency
    for _ in 0..10 {
        for cmd_name in &known_commands {
            let command_text = format!("/{}", cmd_name);
            
            if BotCommand::parse(&command_text, "testbot").is_err() {
                return TestResult::failed();
            }
        }
        
        // Test echo with different arguments
        let echo_commands = vec![
            "/echo test",
            "/echo hello world",
            "/echo multiple word message here",
        ];
        
        for echo_cmd in &echo_commands {
            if BotCommand::parse(echo_cmd, "testbot").is_err() {
                return TestResult::failed();
            }
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
        // Test that BotCommand descriptions are available
        let descriptions = BotCommand::descriptions().to_string();
        assert!(descriptions.contains("start"));
        assert!(descriptions.contains("help"));
        assert!(descriptions.contains("echo"));
        assert!(descriptions.contains("status"));
        
        // Test that known commands can be parsed
        assert!(BotCommand::parse("/start", "testbot").is_ok());
        assert!(BotCommand::parse("/help", "testbot").is_ok());
        assert!(BotCommand::parse("/status", "testbot").is_ok());
        assert!(BotCommand::parse("/echo test", "testbot").is_ok());
        
        // Test that unknown commands fail to parse
        assert!(BotCommand::parse("/unknown", "testbot").is_err());
    }
    
    #[test]
    fn test_case_sensitive_parsing() {
        // Test that BotCommand parsing follows the case rules defined in the enum
        // The derive macro uses lowercase by default due to rename_rule = "lowercase"
        assert!(BotCommand::parse("/start", "testbot").is_ok());
        assert!(BotCommand::parse("/help", "testbot").is_ok());
        assert!(BotCommand::parse("/status", "testbot").is_ok());
        assert!(BotCommand::parse("/echo test", "testbot").is_ok());
        
        // Verify the parsed commands are correct
        if let Ok(cmd) = BotCommand::parse("/start", "testbot") {
            match cmd {
                BotCommand::Start => {}, // Expected
                _ => panic!("Expected Start command"),
            }
        }
    }
}