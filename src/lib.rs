//! Telegram Bot Template Library
//! 
//! This library provides a clean, extensible foundation for building Telegram bots in Rust.
//! It includes a modular architecture with clear separation of concerns and well-defined
//! extension points for adding custom functionality.

// Core modules - these provide the essential bot functionality
pub mod config;    // Configuration management with environment variable support
pub mod error;     // Comprehensive error handling and logging infrastructure
pub mod commands;  // Extensible command handler system with built-in commands
pub mod bot;       // Core bot service layer with Telegram API integration
pub mod ai;        // AI router system with placeholder implementations

// Optional modules for extended functionality
// Uncomment and implement these modules as needed for your specific use case:
// 
// pub mod database;     // Database integration (PostgreSQL, SQLite, etc.)
// pub mod middleware;   // Request/response middleware (rate limiting, auth, etc.)
// pub mod services;     // External service integrations (weather, translation, etc.)
// pub mod analytics;    // Usage analytics and metrics collection
// pub mod cache;        // Caching layer for improved performance
// pub mod webhooks;     // Webhook handling for external integrations

// Re-export commonly used types for convenience
pub use config::Config;
pub use error::{BotError, BotResult};
pub use bot::BotService;
pub use commands::{CommandHandler, CommandRouter};
pub use ai::{AIRouter, AIProcessor};