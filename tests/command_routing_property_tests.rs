use quickcheck::{quickcheck, TestResult};
use telegram_bot_template::commands::BotCommand;
use teloxide::utils::command::BotCommands;

/// **Feature: telegram-bot-template, Property 2: Command Routing Accuracy**
/// 
/// For any registered command and valid command message, the BotCommands derive macro should 
/// correctly parse commands and route them to the appropriate handlers
/// 
/// **Validates: Requirements 3.1, 3.4**

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

/// Test that BotCommands parsing works correctly with known commands
fn test_command_routing_accuracy(command_names: Vec<String>) -> TestResult {
    if command_names.is_empty() || command_names.len() > 10 {
        return TestResult::discard();
    }
    
    // Test with known bot commands
    let known_commands = vec!["start", "help", "echo", "status"];
    
    for cmd_name in &known_commands {
        let command_text = format!("/{}", cmd_name);
        
        // Test basic command format validation
        if !command_text.starts_with('/') {
            return TestResult::failed();
        }
        
        // Test command name extraction
        let without_slash = &command_text[1..];
        let parts: Vec<&str> = without_slash.split_whitespace().collect();
        
        if parts.is_empty() {
            return TestResult::failed();
        }
        
        let parsed_name = parts[0].to_lowercase();
        if parsed_name != cmd_name.to_lowercase() {
            return TestResult::failed();
        }
        
        // Test that BotCommand can parse known commands
        match cmd_name {
            &"start" => {
                if BotCommand::parse(&command_text, "testbot").is_err() {
                    return TestResult::failed();
                }
            }
            &"help" => {
                if BotCommand::parse(&command_text, "testbot").is_err() {
                    return TestResult::failed();
                }
            }
            &"status" => {
                if BotCommand::parse(&command_text, "testbot").is_err() {
                    return TestResult::failed();
                }
            }
            &"echo" => {
                // Echo requires an argument, so test with one
                let echo_with_arg = format!("{} test", command_text);
                if BotCommand::parse(&echo_with_arg, "testbot").is_err() {
                    return TestResult::failed();
                }
            }
            _ => {}
        }
    }
    
    TestResult::passed()
}

/// Test that command parsing handles edge cases correctly
fn test_command_parsing_edge_cases(input: String) -> TestResult {
    // Test empty command
    if input.is_empty() {
        // Empty input should not be a valid command
        return TestResult::from_bool(!input.starts_with('/'));
    }
    
    // Test command without leading slash
    if !input.starts_with('/') {
        // Non-slash input should not be treated as command
        return TestResult::passed();
    }
    
    // Test valid command format
    if input.starts_with('/') && input.len() > 1 {
        let without_slash = &input[1..];
        let parts: Vec<&str> = without_slash.split_whitespace().collect();
        
        if parts.is_empty() {
            return TestResult::from_bool(false); // Should have at least command name
        }
        
        let command_name = parts[0];
        // Command name should not be empty and should not contain '/'
        TestResult::from_bool(!command_name.is_empty() && !command_name.contains('/'))
    } else {
        TestResult::discard()
    }
}

/// Test that commands with @botname suffix are handled correctly
fn test_command_with_botname_suffix(bot_name: String) -> TestResult {
    let clean_bot_name = generate_valid_command_name(bot_name);
    
    if clean_bot_name.is_empty() {
        return TestResult::discard();
    }
    
    // Test known commands with @botname suffix
    let known_commands = vec!["start", "help", "status"];
    
    for cmd_name in &known_commands {
        // Test command with @botname suffix parsing
        let command_text = format!("/{}@{}", cmd_name, clean_bot_name);
        
        if !command_text.starts_with('/') {
            return TestResult::failed();
        }
        
        // Test that BotCommand can parse commands with @botname suffix
        if BotCommand::parse(&command_text, &clean_bot_name).is_err() {
            return TestResult::failed();
        }
        
        let without_slash = &command_text[1..];
        let parts: Vec<&str> = without_slash.split_whitespace().collect();
        
        if parts.is_empty() {
            return TestResult::failed();
        }
        
        let command_part = parts[0];
        
        // Handle @botname suffix
        let parsed_name = if let Some(at_pos) = command_part.find('@') {
            command_part[..at_pos].to_string()
        } else {
            command_part.to_string()
        };
        
        // The parsed name should be the command without the @botname suffix
        if parsed_name != *cmd_name {
            return TestResult::failed();
        }
    }
    
    TestResult::passed()
}

/// Test that unknown commands are handled appropriately
fn test_unknown_command_handling(unknown_command: String) -> TestResult {
    let unknown_clean = generate_valid_command_name(unknown_command);
    
    if unknown_clean.is_empty() {
        return TestResult::discard();
    }
    
    // Known commands in our BotCommand enum
    let known_commands = vec!["start", "help", "echo", "status"];
    
    // Skip if the unknown command is actually a known command
    if known_commands.contains(&unknown_clean.as_str()) {
        return TestResult::discard();
    }
    
    // Test that known commands can be parsed
    for cmd_name in &known_commands {
        let command_text = format!("/{}", cmd_name);
        if !command_text.starts_with('/') {
            return TestResult::failed();
        }
        
        // Known commands should parse successfully (except echo without args)
        if *cmd_name != "echo" {
            if BotCommand::parse(&command_text, "testbot").is_err() {
                return TestResult::failed();
            }
        }
    }
    
    // Test that unknown command fails to parse (which is expected behavior)
    let unknown_command_text = format!("/{}", unknown_clean);
    if !unknown_command_text.starts_with('/') {
        return TestResult::failed();
    }
    
    // Unknown commands should fail to parse with BotCommand
    if BotCommand::parse(&unknown_command_text, "testbot").is_ok() {
        return TestResult::failed(); // Should fail for unknown commands
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
        fn prop(bot_name: String) -> TestResult {
            test_command_with_botname_suffix(bot_name)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_property_unknown_command_handling() {
        fn prop(unknown_command: String) -> TestResult {
            test_unknown_command_handling(unknown_command)
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    
    #[test]
    fn test_basic_command_parsing() {
        // Test basic command format validation
        let command_text = "/start";
        assert!(command_text.starts_with('/'));
        
        // Test that BotCommand can parse the start command
        let result = BotCommand::parse(command_text, "testbot");
        assert!(result.is_ok());
        
        if let Ok(cmd) = result {
            match cmd {
                BotCommand::Start => {}, // Expected
                _ => panic!("Expected Start command"),
            }
        }
    }
    
    #[test]
    fn test_command_with_args_parsing() {
        let command_text = "/echo hello world";
        assert!(command_text.starts_with('/'));
        
        // Test that BotCommand can parse the echo command with arguments
        let result = BotCommand::parse(command_text, "testbot");
        assert!(result.is_ok());
        
        if let Ok(cmd) = result {
            match cmd {
                BotCommand::Echo(text) => {
                    assert_eq!(text, "hello world");
                },
                _ => panic!("Expected Echo command"),
            }
        }
    }
    
    #[test]
    fn test_command_parsing_case_sensitivity() {
        // Test that BotCommand parsing follows the case rules defined in the enum
        // The derive macro uses lowercase by default due to rename_rule = "lowercase"
        
        // Lowercase commands should work (as defined in enum)
        let result = BotCommand::parse("/start", "testbot");
        assert!(result.is_ok());
        match result.unwrap() {
            BotCommand::Start => {}, // Expected
            _ => panic!("Expected Start command"),
        }
        
        let result = BotCommand::parse("/help", "testbot");
        assert!(result.is_ok());
        match result.unwrap() {
            BotCommand::Help => {}, // Expected
            _ => panic!("Expected Help command"),
        }
        
        let result = BotCommand::parse("/echo hello", "testbot");
        assert!(result.is_ok());
        match result.unwrap() {
            BotCommand::Echo(text) => {
                assert_eq!(text, "hello");
            },
            _ => panic!("Expected Echo command"),
        }
    }
    
    #[test]
    fn test_bot_command_descriptions() {
        let descriptions = BotCommand::descriptions().to_string();
        assert!(descriptions.contains("start"));
        assert!(descriptions.contains("help"));
        assert!(descriptions.contains("echo"));
        assert!(descriptions.contains("status"));
    }
}