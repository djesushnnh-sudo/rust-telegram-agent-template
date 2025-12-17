//! Telegram Bot Template Library
//! 
//! Enhanced with patterns from production Telegram bots, this library provides a robust,
//! scalable foundation for building sophisticated Telegram bots in Rust. Features include
//! concurrent state management, AI routing, and comprehensive database integration.

// Core modules - these provide the essential bot functionality
pub mod config;    // Simple configuration management with .env file
pub mod error;     // Comprehensive error handling and logging infrastructure
pub mod commands;  // Modern command system using BotCommands derive macro
pub mod state;     // Concurrent state management with DashMap
pub mod ai;        // Enhanced AI router with session management

// Database integration
pub mod database;  // SQLite database adapter with state persistence

// Re-export commonly used types for convenience
pub use config::Config;
pub use error::{BotError, BotResult};
pub use state::AppState;
pub use commands::{BotCommand, CommandHandler};
pub use ai::AIProcessor;