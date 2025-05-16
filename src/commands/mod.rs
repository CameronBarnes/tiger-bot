use db::deadpool_postgres::{Object, Pool};
use nanoid::nanoid;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommands as commands;
use tracing::{debug, error, warn};

use crate::utils::is_user_opt_out;

pub mod quotes;

#[derive(BotCommands, Clone, Copy)]
#[command(rename_rule = "lowercase")]
pub enum GeneralCommand {
    #[command(description = "Prints command descriptions")]
    Help,
    #[command(
        description = "Opt out or in to quote functionality, removes existing quotes made by or from this user"
    )]
    ToggleOptOut,
}

pub async fn endpoint(
    db_pool: Pool,
    cache_pool: ConnectionManager,
    bot: Bot,
    msg: Message,
    cmd: GeneralCommand,
) -> Result<(), teloxide::RequestError> {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to get DB client with err: {err:?}");
            return Ok(());
        }
    };
    match cmd {
        GeneralCommand::Help => {
            let mut text = String::from("Commands for TigerTM Bot\n\nGeneral:\n");
            for cmd in GeneralCommand::bot_commands() {
                text.push_str(&format!("{} - {}\n", cmd.command, cmd.description));
            }
            text.push_str("\nQuotes:\n");
            for cmd in quotes::Command::bot_commands() {
                text.push_str(&format!("{} - {}\n", cmd.command, cmd.description));
            }

            bot.send_message(msg.chat.id, text)
                .reply_to(msg)
                .await
                .map(|_| ())
        }
        GeneralCommand::ToggleOptOut => handle_opt_out_cmd(&client, cache_pool, bot, &msg).await,
    }
}

async fn handle_opt_out_cmd(
    client: &Object,
    mut cache_pool: ConnectionManager,
    bot: Bot,
    msg: &Message,
) -> Result<(), teloxide::RequestError> {
    let Some(user) = msg.from.as_ref() else {
        warn!("Failed to get user for opt out cmd");
        return Ok(());
    };
    let has_opted_out = is_user_opt_out(client, user.id.0).await;
    let id = nanoid!();
    match cache_pool
        .set::<&std::string::String, u64, String>(&id, user.id.0)
        .await
    {
        Ok(val) => {
            debug!("Returned value from cache: {val}");
        }
        Err(err) => {
            error!("Cache error: {err:?}");
            return bot
                .send_message(
                    msg.chat.id,
                    "Sorry, I ran into an error, please try again later.",
                )
                .reply_to(msg)
                .await
                .map(|_| ());
        }
    }

    let (text, keyboard) = if has_opted_out {
        let text = "You are currently opted out of quote functionality.\nWould you like to opt in? This bot only stores the quotes and your username.\nNo other data is stored.";
        let yes_text = format!("opt_in_yes||{id}");
        assert!(yes_text.len() <= 64);
        let btn_yes = InlineKeyboardButton::callback("Yes, Let Me use Quotes", yes_text);
        let no_text = format!("opt_in_no||{id}");
        assert!(no_text.len() <= 64);
        let btn_no = InlineKeyboardButton::callback("No, No Quotes for Me", no_text);
        let keyboard = InlineKeyboardMarkup::default().append_row(vec![btn_yes, btn_no]);
        (text, keyboard)
    } else {
        let text = "This will prevent you from being quoted and quoting other's messages.\nIt will also delete all existing quotes from you, and quotes you added.\nAre you sure?";
        let yes_text = format!("opt_out_yes||{id}");
        assert!(yes_text.len() <= 64);
        let btn_yes = InlineKeyboardButton::callback("Yes, Delete My Data", yes_text);
        let no_text = format!("opt_out_no||{id}");
        assert!(no_text.len() <= 64);
        let btn_no = InlineKeyboardButton::callback("No, Keep My Quotes", no_text);
        let keyboard = InlineKeyboardMarkup::default().append_row(vec![btn_yes, btn_no]);
        (text, keyboard)
    };
    bot.send_message(msg.chat.id, text)
        .reply_to(msg)
        .reply_markup(keyboard)
        .await
        .map(|_| ())
}
