pub mod bot;
pub mod config;
pub mod error;

// Re-exporting public API
pub use bot::{handle_command, Command};
pub use config::BotConfig;
pub use error::{BotError, BotResult};