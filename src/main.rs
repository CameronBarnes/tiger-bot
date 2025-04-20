use std::net::SocketAddr;

use commands::quotes;
use config::Config;
use db::{
    deadpool_postgres::{Config as DBConfig, CreatePoolError, Pool},
    tokio_postgres::NoTls,
};
use teloxide::{prelude::*, update_listeners::webhooks, utils::command::BotCommands};
use tracing::{info, warn};

mod commands;
mod config;
#[allow(clippy::disallowed_types)]
mod utils;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    info!("Hello Mom!");

    let config = Config::new();
    let pool = create_pool(&config).await.expect("Failed to connect to DB");

    let bot = Bot::from_env();

    let _ = bot
        .set_my_commands(
            quotes::Command::bot_commands()
                .into_iter()
                .chain(commands::GeneralCommand::bot_commands()),
        )
        .await;

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<commands::GeneralCommand>()
                .endpoint(commands::endpoint),
        )
        .branch(
            dptree::entry()
                .filter_command::<quotes::Command>()
                .endpoint(quotes::endpoint),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(
            addr,
            config.url.clone().parse().expect("Failed to parse URL"),
        ),
    )
    .await
    .expect("Failed to setup webhook");

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
        .default_handler(|update| async move {
            warn!("Unhandled update: {update:?}");
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, LoggingErrorHandler::new())
        .await;
}

async fn create_pool(config: &Config) -> Result<Pool, CreatePoolError> {
    let mut cfg = DBConfig::new();
    cfg.dbname = Some(config.database_name.clone());
    cfg.user = Some(config.database_user.clone());
    cfg.host = Some(config.database_host.clone());
    cfg.port = Some(config.database_port);
    cfg.password = Some(config.database_password.clone());
    cfg.create_pool(Some(db::deadpool_postgres::Runtime::Tokio1), NoTls)
}
