use anyhow::Result;
use log::{info, error};
use tokio::signal;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

// Import all the modules we need for bot operation
use telegram_bot_template::config::Config;
use telegram_bot_template::state::AppState;
use telegram_bot_template::commands::{BotCommand, CommandHandler};
use telegram_bot_template::ai::AIProcessor;
use telegram_bot_template::error::LoggingConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from .env file
    let config = match Config::from_env() {
        Ok(config) => {
            println!("✅ Configuration loaded successfully");
            config
        }
        Err(e) => {
            eprintln!("❌ Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize logging after loading config (so RUST_LOG is available)
    if let Err(e) = LoggingConfig::init() {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }
    
    info!("🚀 Starting Telegram Bot Template...");
    
    // Validate bot token
    let bot_token = config.bot_token.clone();
    if bot_token.is_empty() {
        error!("❌ FATAL: TELEGRAM_BOT_TOKEN is not set");
        error!("Please set TELEGRAM_BOT_TOKEN in your environment or .env file");
        std::process::exit(1);
    }

    // Initialize shared application state
    let state = Arc::new(AppState::new());
    
    // Load data from database on startup (like Dads2Dads loads from Airtable)
    info!("📥 Loading persistent data...");
    if let Err(e) = state.load_from_database().await {
        error!("❌ Failed to load data from database: {}", e);
        error!("⚠️ Continuing with empty state. Data will not persist across restarts.");
    }

    // Initialize bot
    let bot = Bot::new(bot_token);
    
    // Initialize AI processor with state
    let ai_processor = Arc::new(AIProcessor::new(
        config.ai_enabled,
        None, // TODO: Add AI API key to config
        state.clone(),
    ));

    // Initialize command handler with state
    let command_handler = Arc::new(CommandHandler::new(state.clone()));

    // Create message handler using Dads2Dads patterns
    let handler = create_message_handler(command_handler.clone(), ai_processor.clone(), state.clone());

    info!("🤖 Bot initialized successfully");
    info!("📋 Available commands: {}", BotCommand::descriptions());

    // Create dispatcher
    let mut dispatcher = Dispatcher::builder(bot, handler).build();

    // Set up graceful shutdown handling
    let shutdown_signal = setup_shutdown_signal();
    
    info!("✅ Bot initialization complete, starting dispatcher...");
    
    // Start the bot with graceful shutdown handling
    tokio::select! {
        _ = dispatcher.dispatch() => {
            info!("Bot dispatcher exited");
        }
        _ = shutdown_signal => {
            info!("Shutdown signal received, stopping bot gracefully");
        }
    }
    
    // Save state before shutdown
    info!("💾 Saving state before shutdown...");
    if let Err(e) = state.save_to_database().await {
        error!("❌ Failed to save state: {}", e);
    }
    
    info!("✅ Telegram Bot Template shutdown complete");
    Ok(())
}

/// Create message handler using Dads2Dads patterns
/// 
/// This creates a sophisticated message handler that separates commands from regular messages,
/// similar to the dual-bot pattern in Dads2Dads but simplified for a single bot template.
fn create_message_handler(
    command_handler: Arc<CommandHandler>,
    ai_processor: Arc<AIProcessor>,
    state: Arc<AppState>,
) -> teloxide::dispatching::UpdateHandler<teloxide::RequestError> {
    use teloxide::dptree;

    // Clone handlers for use in closures
    let command_handler_for_commands = command_handler.clone();
    let command_handler_for_messages = command_handler.clone();
    let ai_processor_for_messages = ai_processor.clone();

    // Create handler branches similar to Dads2Dads
    dptree::entry()
        // Handle commands with type-safe parsing
        .branch(
            Update::filter_message()
                .filter_command::<BotCommand>()
                .endpoint(move |bot: Bot, msg: Message, cmd: BotCommand| {
                    let handler = command_handler_for_commands.clone();
                    async move {
                        if let Err(e) = handler.handle_command(bot, msg, cmd).await {
                            log::error!("Command handling error: {}", e);
                        }
                        teloxide::prelude::ResponseResult::Ok(())
                    }
                })
        )
        // Handle regular messages (for AI routing or general responses)
        .branch(
            Update::filter_message()
                .endpoint(move |bot: Bot, msg: Message| {
                    let handler = command_handler_for_messages.clone();
                    let ai = ai_processor_for_messages.clone();
                    async move {
                        // Check if message should go to AI
                        if ai.should_process_with_ai(&msg).await {
                            // Route to AI processor
                            match ai.process_message_with_state(&msg).await {
                                Ok(response) => {
                                    if let Err(e) = bot.send_message(msg.chat.id, response).await {
                                        log::error!("Failed to send AI response: {}", e);
                                    }
                                }
                                Err(e) => {
                                    log::error!("AI processing error: {}", e);
                                    let _ = bot.send_message(msg.chat.id, "Sorry, I encountered an error processing your message.").await;
                                }
                            }
                        } else {
                            // Handle as regular message
                            if let Err(e) = handler.handle_message(bot, msg).await {
                                log::error!("Message handling error: {}", e);
                            }
                        }
                        teloxide::prelude::ResponseResult::Ok(())
                    }
                })
        )
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