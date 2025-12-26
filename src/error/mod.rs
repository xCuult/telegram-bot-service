//! Centralized error handling using `thiserror`

use thiserror::Error;

/// Listing all possible errors in the bot application
/// 
/// Uses derive macro `thiserror` for automatic implementation
/// `std::error::Error`
/// 
/// - Use `#[from]` for automatic conversion from external types
#[derive(Error, Debug)]
pub enum BotError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Telegram API error
    #[error("Telegram API error: {0}")]
    Telegram(#[from] teloxide::RequestError),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Type alias for Result with our error type
pub type BotResult<T> = Result<T, BotError>;