//! Public API for the bot module.

pub mod commands;
pub mod handler;

// Re-exporting for easier access
pub use commands::Command;
pub use handler::handle_command;