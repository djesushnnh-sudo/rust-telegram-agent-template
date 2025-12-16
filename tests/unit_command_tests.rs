use telegram_bot_template::commands::{CommandRouter, CommandHandler};
use telegram_bot_template::error::BotError;
use async_trait::async_trait;
use teloxide::{Bot, types::Message};

/// Simple mock command handler for testing
struct MockCommandHandler {
    name: String,
    description: String,
    usage: String,
    requires_args: bool,
    min_args: usize,
    max_args: Option<usize>,
}

impl MockCommandHandler {
    fn new() -> Self {
        Self {
            name: "mock".to_string(),
            description: "A mock command for testing".to_string(),
            usage: "/mock - A test command".to_string(),
            requires_args: false,
            min_args: 0,
            max_args: None,
        }
    }
    
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    
    fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }
    
    fn with_usage(mut self, usage: &str) -> Self {
        self.usage = usage.to_string();
        self
    }
    
    fn with_args_requirement(mut self, requires_args: bool, min_args: usize, max_args: Option<usize>) -> Self {
        self.requires_args = requires_args;
        self.min_args = min_args;
        self.max_args = max_args;
        self
    }
}

#[async_trait]
impl CommandHandler for MockCommandHandler {
    async fn handle(&self, _bot: &Bot, _message: &Message, _args: Vec<String>) -> telegram_bot_template::error::BotResult<()> {
        // Mock implementation - just return Ok
        Ok(())
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn usage(&self) -> &str {
        &self.usage
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn requires_args(&self) -> bool {
        self.requires_args
    }
    
    fn min_args(&self) -> usize {
        self.min_args
    }
    
    fn max_args(&self) -> Option<usize> {
        self.max_args
    }
}

/// Unit tests for command registration and basic routing
/// 
/// These tests focus on specific scenarios for command registration,
/// parsing, and routing that complement the property-based tests.

#[test]
fn test_empty_router_creation() {
    let router = CommandRouter::new();
    
    assert_eq!(router.command_count(), 0);
    assert!(router.get_registered_commands().is_empty());
    assert!(!router.is_command_registered("any_command"));
}

#[test]
fn test_single_command_registration() {
    let mut router = CommandRouter::new();
    let handler = Box::new(MockCommandHandler::new().with_name("test"));
    
    router.register_command("test", handler);
    
    assert_eq!(router.command_count(), 1);
    assert!(router.is_command_registered("test"));
    assert!(router.is_command_registered("TEST")); // Case insensitive
    assert!(!router.is_command_registered("nonexistent"));
    
    let commands = router.get_registered_commands();
    assert_eq!(commands.len(), 1);
    assert!(commands.contains(&"test".to_string()));
}

#[test]
fn test_multiple_command_registration() {
    let mut router = CommandRouter::new();
    
    router.register_command("start", Box::new(MockCommandHandler::new().with_name("start")));
    router.register_command("help", Box::new(MockCommandHandler::new().with_name("help")));
    router.register_command("echo", Box::new(MockCommandHandler::new().with_name("echo")));
    
    assert_eq!(router.command_count(), 3);
    assert!(router.is_command_registered("start"));
    assert!(router.is_command_registered("help"));
    assert!(router.is_command_registered("echo"));
    
    let commands = router.get_registered_commands();
    assert_eq!(commands.len(), 3);
    assert!(commands.contains(&"start".to_string()));
    assert!(commands.contains(&"help".to_string()));
    assert!(commands.contains(&"echo".to_string()));
}

#[test]
fn test_command_registration_case_insensitive() {
    let mut router = CommandRouter::new();
    
    // Register with mixed case
    router.register_command("TestCommand", Box::new(MockCommandHandler::new().with_name("TestCommand")));
    
    // Should be stored in lowercase
    assert!(router.is_command_registered("testcommand"));
    assert!(router.is_command_registered("TESTCOMMAND"));
    assert!(router.is_command_registered("TestCommand"));
    
    let commands = router.get_registered_commands();
    assert!(commands.contains(&"testcommand".to_string()));
}

#[test]
fn test_command_overwrite() {
    let mut router = CommandRouter::new();
    
    // Register initial command
    router.register_command("test", Box::new(MockCommandHandler::new().with_name("test1")));
    assert_eq!(router.command_count(), 1);
    
    // Register same command name again (should overwrite)
    router.register_command("test", Box::new(MockCommandHandler::new().with_name("test2")));
    assert_eq!(router.command_count(), 1); // Still only one command
    assert!(router.is_command_registered("test"));
}

#[test]
fn test_command_parsing_basic() {
    let router = CommandRouter::new();
    
    // Basic command without arguments
    let result = router.parse_command("/start");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "start");
    assert!(args.is_empty());
}

#[test]
fn test_command_parsing_with_arguments() {
    let router = CommandRouter::new();
    
    // Command with multiple arguments
    let result = router.parse_command("/echo hello world test");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "echo");
    assert_eq!(args, vec!["hello", "world", "test"]);
}

#[test]
fn test_command_parsing_with_botname() {
    let router = CommandRouter::new();
    
    // Command with @botname suffix
    let result = router.parse_command("/start@mybotname");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "start");
    assert!(args.is_empty());
    
    // Command with @botname and arguments
    let result = router.parse_command("/echo@mybotname hello world");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "echo");
    assert_eq!(args, vec!["hello", "world"]);
}

#[test]
fn test_command_parsing_case_normalization() {
    let router = CommandRouter::new();
    
    // Mixed case command should be normalized to lowercase
    let result = router.parse_command("/START");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "start");
    assert!(args.is_empty());
    
    let result = router.parse_command("/EcHo@BotName Hello World");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "echo");
    assert_eq!(args, vec!["Hello", "World"]); // Args preserve case
}

#[test]
fn test_command_parsing_whitespace_handling() {
    let router = CommandRouter::new();
    
    // Multiple spaces between arguments
    let result = router.parse_command("/echo   hello    world   ");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "echo");
    assert_eq!(args, vec!["hello", "world"]);
    
    // Leading/trailing spaces - the current implementation doesn't handle leading spaces
    // before the slash, so this will fail
    let result = router.parse_command("  /start  ");
    assert!(result.is_err()); // Leading spaces before / are not handled
}

#[test]
fn test_command_parsing_invalid_format() {
    let router = CommandRouter::new();
    
    // No leading slash
    let result = router.parse_command("start");
    assert!(result.is_err());
    if let Err(BotError::MessageParsing(msg)) = result {
        assert!(msg.contains("must start with"));
    } else {
        panic!("Expected MessageParsing error");
    }
    
    // Empty command (just slash)
    let result = router.parse_command("/");
    assert!(result.is_err());
    if let Err(BotError::MessageParsing(msg)) = result {
        assert!(msg.contains("Empty command"));
    } else {
        panic!("Expected MessageParsing error");
    }
    
    // Only whitespace after slash
    let result = router.parse_command("/   ");
    assert!(result.is_err());
}

#[test]
fn test_command_parsing_edge_cases() {
    let router = CommandRouter::new();
    
    // Command with only @botname (no actual command)
    let result = router.parse_command("/@botname");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert!(cmd.is_empty()); // Empty command name after removing @botname
    assert!(args.is_empty());
    
    // Command with multiple @ symbols
    let result = router.parse_command("/test@bot@name hello");
    assert!(result.is_ok());
    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "test"); // Only first @ is considered
    assert_eq!(args, vec!["hello"]);
}

#[test]
fn test_default_router() {
    let router = CommandRouter::default();
    
    assert_eq!(router.command_count(), 0);
    assert!(router.get_registered_commands().is_empty());
}

// Test command handler argument validation
#[test]
fn test_command_handler_argument_validation() {
    let handler = MockCommandHandler::new()
        .with_args_requirement(true, 2, Some(4));
    
    // Test with correct number of arguments
    let args = vec!["arg1".to_string(), "arg2".to_string()];
    assert!(handler.validate_args(&args).is_ok());
    
    let args = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()];
    assert!(handler.validate_args(&args).is_ok());
    
    // Test with too few arguments
    let args = vec!["arg1".to_string()];
    let result = handler.validate_args(&args);
    assert!(result.is_err());
    if let Err(BotError::Command(msg)) = result {
        assert!(msg.contains("requires at least"));
    } else {
        panic!("Expected Command error for too few arguments");
    }
    
    // Test with too many arguments
    let args = vec![
        "arg1".to_string(),
        "arg2".to_string(),
        "arg3".to_string(),
        "arg4".to_string(),
        "arg5".to_string(),
    ];
    let result = handler.validate_args(&args);
    assert!(result.is_err());
    if let Err(BotError::Command(msg)) = result {
        assert!(msg.contains("accepts at most"));
    } else {
        panic!("Expected Command error for too many arguments");
    }
}

#[test]
fn test_command_handler_no_argument_limits() {
    let handler = MockCommandHandler::new(); // Default: no limits
    
    // Should accept any number of arguments
    assert!(handler.validate_args(&[]).is_ok());
    assert!(handler.validate_args(&["one".to_string()]).is_ok());
    assert!(handler.validate_args(&["one".to_string(), "two".to_string()]).is_ok());
    
    let many_args: Vec<String> = (0..100).map(|i| format!("arg{}", i)).collect();
    assert!(handler.validate_args(&many_args).is_ok());
}

#[test]
fn test_command_handler_metadata() {
    let handler = MockCommandHandler::new()
        .with_name("test_command")
        .with_description("A test command for unit testing")
        .with_usage("/test_command <arg1> [arg2] - Test command usage");
    
    assert_eq!(handler.name(), "test_command");
    assert_eq!(handler.description(), "A test command for unit testing");
    assert_eq!(handler.usage(), "/test_command <arg1> [arg2] - Test command usage");
    assert!(!handler.requires_args()); // Default
    assert_eq!(handler.min_args(), 0); // Default
    assert_eq!(handler.max_args(), None); // Default
}