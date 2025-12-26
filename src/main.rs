//! Entry Point
//! 
//! - Runtime init
//! - Logging setup
//! - Run the bot
//! 
//! # Exit codes
//! 
//! `0` - Graceful shutdown
//! `1` - Configuration error
//! `2` - Runtime error

use tg_bot_service::{handle_command, BotConfig, Command};
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Init logging
fn init_logging(log_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .with(filter)
        .init();
}

/// Starts the bot and processes incoming messages
async fn run_bot(config: BotConfig) -> anyhow::Result<()> {
    info!("Starting the bot...");

    // Create a bot instance with a token from the config
    let bot = Bot::new(config.token());

    // Logging bot info
    let me = bot.get_me().await?;
    info!(
        bot_username = %me.username(),
        "Bot initialized successfully!"
    );

    // Register bot commands
    bot.set_my_commands(Command::bot_commands()).await?;
    info!("Bot commands registered!");

    // Create dispatcher and register command handler
    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(handle_command);

    // Run the bot
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    info!("Bot stopped gracefully!");
    Ok(())
}

/// Entry point
#[tokio::main]
async fn main() {
    let config = match BotConfig::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize logging
    init_logging(config.log_level());

    // Run the bot
    if let Err(e) = run_bot(config).await {
        error!(error = %e, "Bot crashed!");
        std::process::exit(2);
    }
}