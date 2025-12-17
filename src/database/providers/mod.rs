//! Database provider implementations
//! 
//! This module contains different database provider implementations that all
//! implement the `DatabaseProvider` trait. This allows the bot to be database-agnostic
//! and easily switch between different storage backends.

pub mod sqlite;
pub mod memory;

pub use sqlite::SqliteProvider;
pub use memory::InMemoryProvider;