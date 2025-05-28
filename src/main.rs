use commands::quotes;
use config::Config;
use db::{
    deadpool_postgres::{Config as DBConfig, CreatePoolError, Pool},
    tokio_postgres::NoTls,
};
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{debug, info};

mod callbacks;
mod commands;
mod config;
#[allow(clippy::disallowed_types)]
mod utils;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    info!("Hello Mom!");

    info!("Loading configuration");
    let config = Config::new();
    info!("Creating Database connection pool");
    let db_pool = create_pool(&config).await.expect("Failed to connect to DB");
    info!("Creating connection manager for cache");
    let client = redis::Client::open(format!("redis://{}", &config.cache_host))
        .expect("Failed to connect to cache");
    let cache_pool = client
        .get_connection_manager()
        .await
        .expect("Failed to construct cache connection manager");

    let bot = Bot::from_env();

    let _ = bot
        .set_my_commands(
            quotes::Command::bot_commands()
                .into_iter()
                .chain(commands::GeneralCommand::bot_commands()),
        )
        .await;

    let message_handler = Update::filter_message()
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

    let callback_handler = Update::filter_callback_query()
        // .filter(|callback: CallbackQuery| callback.message.is_some())
        .endpoint(callbacks::endpoint);

    let handler = dptree::entry()
        .branch(message_handler)
        .branch(callback_handler);

    info!("Created message and callback handler");

    // Unable to use webhook due to issue with callbacks
    /*info!("Creating webhook listener");
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(
            addr,
            config.url.clone().parse().expect("Failed to parse URL"),
        ),
    )
    .await
    .expect("Failed to setup webhook");*/

    info!("Starting dispatcher");
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db_pool, cache_pool])
        .default_handler(async |update| debug!("Unhandled update: {update:?}"))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
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
