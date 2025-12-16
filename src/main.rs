use anyhow::Result;
use log::{info, error};
use tokio::signal;

// Import all the modules we need for bot operation
use telegram_bot_template::config::Config;
use telegram_bot_template::bot::BotService;
use telegram_bot_template::commands::{StartCommand, HelpCommand, EchoCommand, StatusCommand};
use telegram_bot_template::error::LoggingConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging first
    if let Err(e) = LoggingConfig::init() {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }
    
    info!("Starting Telegram Bot Template");
    
    // Load and validate configuration from environment variables
    let config = match Config::from_env() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e.into());
        }
    };
    
    // Initialize bot service with configuration
    let mut bot_service = match BotService::new(&config).await {
        Ok(service) => {
            info!("Bot service initialized successfully");
            service
        }
        Err(e) => {
            error!("Failed to initialize bot service: {}", e);
            return Err(e.into());
        }
    };
    
    // Register built-in command handlers
    register_commands(&mut bot_service, &config).await?;
    
    // Set up graceful shutdown handling
    let shutdown_signal = setup_shutdown_signal();
    
    info!("Bot initialization complete, starting main loop");
    
    // Start the bot with graceful shutdown handling
    tokio::select! {
        result = bot_service.start() => {
            match result {
                Ok(()) => {
                    info!("Bot stopped normally");
                }
                Err(e) => {
                    error!("Bot stopped with error: {}", e);
                    return Err(e.into());
                }
            }
        }
        _ = shutdown_signal => {
            info!("Shutdown signal received, stopping bot gracefully");
            if let Err(e) = bot_service.shutdown().await {
                error!("Error during shutdown: {}", e);
            }
        }
    }
    
    info!("Telegram Bot Template shutdown complete");
    Ok(())
}

/// Register all built-in command handlers with the bot service
/// 
/// This function demonstrates the command registration pattern and serves as the
/// central location for adding new commands to your bot.
/// 
/// ## Adding Custom Commands
/// 
/// To add a new command to your bot:
/// 
/// 1. **Create the command file** in `src/commands/` (e.g., `weather.rs`)
/// 2. **Implement the CommandHandler trait** for your command struct
/// 3. **Export the command** in `src/commands/mod.rs`
/// 4. **Register it here** following the patterns below
/// 
/// ### Example Command Registrations:
/// 
/// ```rust
/// // Simple command without configuration
/// router.register_command("weather", Box::new(WeatherCommand::new()));
/// 
/// // Command that needs API key from configuration
/// if let Some(api_key) = &config.weather_api_key {
///     router.register_command("weather", Box::new(WeatherCommand::new(api_key.clone())));
/// }
/// 
/// // Command with conditional registration based on feature flags
/// if config.enable_ai_chat {
///     router.register_command("chat", Box::new(AIChatCommand::new(&config)));
/// }
/// 
/// // Command that needs database access
/// if let Some(db_pool) = &database_pool {
///     router.register_command("profile", Box::new(ProfileCommand::new(db_pool.clone())));
/// }
/// ```
/// 
/// ### Command Categories to Consider:
/// 
/// - **Utility Commands**: `/weather`, `/translate`, `/qr`, `/shorten`
/// - **Information Commands**: `/news`, `/define`, `/search`, `/wiki`
/// - **Interactive Commands**: `/poll`, `/quiz`, `/game`, `/remind`
/// - **AI Commands**: `/chat`, `/ask`, `/summarize`, `/explain`
/// - **Admin Commands**: `/stats`, `/users`, `/broadcast`, `/maintenance`
async fn register_commands(bot_service: &mut BotService, _config: &Config) -> Result<()> {
    info!("Registering built-in commands");
    
    let router = bot_service.command_router_mut();
    
    // Register core commands - these serve as examples for creating new commands
    router.register_command("start", Box::new(StartCommand::new()));
    router.register_command("help", Box::new(HelpCommand::new()));
    router.register_command("echo", Box::new(EchoCommand::new()));
    router.register_command("status", Box::new(StatusCommand::new()));
    
    // =========================================================================
    // CUSTOM COMMAND REGISTRATION
    // =========================================================================
    // Add your custom commands here following the patterns above.
    // 
    // Uncomment and modify these examples as needed:
    
    // Example: Weather command (requires API key)
    // if let Some(api_key) = std::env::var("WEATHER_API_KEY").ok() {
    //     router.register_command("weather", Box::new(WeatherCommand::new(api_key)));
    // }
    
    // Example: AI chat command (requires AI to be enabled)
    // if config.ai_enabled {
    //     router.register_command("chat", Box::new(AIChatCommand::new()));
    // }
    
    // Example: Database-dependent command
    // if let Some(db_pool) = &database_pool {
    //     router.register_command("profile", Box::new(ProfileCommand::new(db_pool.clone())));
    //     router.register_command("settings", Box::new(SettingsCommand::new(db_pool.clone())));
    // }
    
    // Example: Admin-only commands
    // router.register_command("stats", Box::new(StatsCommand::new()));
    // router.register_command("broadcast", Box::new(BroadcastCommand::new()));
    
    // =========================================================================
    
    info!("Registered {} commands", router.command_count());
    
    // Log available commands for debugging
    let commands = router.get_registered_commands();
    info!("Available commands: {}", commands.join(", "));
    
    Ok(())
}

/// Set up graceful shutdown signal handling
async fn setup_shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received terminate signal");
        },
    }
}