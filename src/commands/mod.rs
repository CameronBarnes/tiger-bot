use db::deadpool_postgres::Pool;
use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::utils::command::BotCommands as commands;

pub mod quotes;

#[derive(BotCommands, Clone, Copy)]
#[command(rename_rule = "lowercase")]
pub enum GeneralCommand {
    #[command(description = "Prints this help text")]
    Help,
}

pub async fn endpoint(
    _db_pool: Pool,
    bot: Bot,
    msg: Message,
    cmd: GeneralCommand,
) -> Result<(), teloxide::RequestError> {
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
    }
}
