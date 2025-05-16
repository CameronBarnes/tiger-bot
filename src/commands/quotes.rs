use db::{
    deadpool_postgres::{Object, Pool},
    queries::quotes::Quote,
    tokio_postgres::{self, error::SqlState},
    types::QuoteType,
};
use itertools::Itertools;
use rust_decimal::{Decimal, prelude::FromPrimitive, prelude::ToPrimitive};
use teloxide::{
    macros::BotCommands,
    prelude::*,
    sugar::request::RequestReplyExt,
    types::{InputFile, MessageKind, User},
};
use tracing::{error, warn};

use crate::utils::{get_username, is_user_opt_out};

#[derive(Debug, thiserror::Error)]
pub(crate) enum QuoteError {
    #[error("Request to Telegram failed with error: {0:?}")]
    TeloxideRequest(teloxide::RequestError),
    #[error("Telegram Data is Missing a FileID for message: {0:?}")]
    TGMissingFileID(Box<Message>),
    #[error("Database Error: {0:?}")]
    Database(tokio_postgres::error::Error),
    #[error("Database Unique Violation: {0:?}")]
    DatabaseUniqueViolation(tokio_postgres::error::Error),
    #[error("No Results from Database: {0:?}")]
    DatabaseNoResults(tokio_postgres::error::Error),
}

impl From<tokio_postgres::error::Error> for QuoteError {
    fn from(err: tokio_postgres::error::Error) -> Self {
        if err
            .as_db_error()
            .is_some_and(|db_error| *db_error.code() == SqlState::UNIQUE_VIOLATION)
        {
            Self::DatabaseUniqueViolation(err)
        } else if format!("{err:?}").eq("Error { kind: RowCount, cause: None }") {
            Self::DatabaseNoResults(err)
        } else {
            Self::Database(err)
        }
    }
}

impl From<teloxide::RequestError> for QuoteError {
    fn from(err: teloxide::RequestError) -> Self {
        Self::TeloxideRequest(err)
    }
}

#[derive(BotCommands, Clone, Copy)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Quote a message and add it to the database")]
    Quote,
    #[command(description = "Get a random quote from the database")]
    RandomQuote,
    #[command(description = "Search for a quote in the database")]
    SearchQuote,
    #[command(description = "Display quote rankings and statistics")]
    QuoteRankings,
}

pub async fn endpoint(
    db_pool: Pool,
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> Result<(), teloxide::RequestError> {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to get DB client with err: {err:?}");
            return Ok(());
        }
    };
    match cmd {
        Command::Quote => quote(&client, msg, bot).await,
        Command::SearchQuote => search_quote(&client, msg, bot).await,
        Command::QuoteRankings => get_quote_statistics(&client, msg, bot).await,
        Command::RandomQuote => get_random_quote(&client, msg, bot).await,
    }
}

async fn quote(client: &Object, msg: Message, bot: Bot) -> Result<(), teloxide::RequestError> {
    // The quote command needs a replied to message to quote
    let text = if let Some(quote) = msg.reply_to_message() {
        if let Some(from) = &msg.from {
            if let Some(quote_from) = &quote.from {
                // The quote must have a valid user object and not be from a channel
                // Command also must not be run by a user who has opted out, or be a quote of a
                // user who opted out
                if is_user_opt_out(client, from.id.0).await {
                    String::from(
                        "You have opted out of quote functionality. Please opt in if you wish to quote messages again.",
                    )
                } else if is_user_opt_out(client, quote_from.id.0).await {
                    String::from("The user you are trying to quote has opted out.")
                } else if quote_from.is_bot {
                    String::from("Cant quote a bot's messages.")
                } else if quote_from.is_telegram()
                    || quote_from.is_channel()
                    || quote_from.is_anonymous()
                {
                    String::from("Cant quote a channel's messages.")
                } else {
                    let res = add_quote(client, quote, from).await;
                    if let Err(err) = &res {
                        // If the quote was already added, we're going to handle this
                        // seperately
                        if err
                            .as_db_error()
                            .is_some_and(|db_error| *db_error.code() == SqlState::UNIQUE_VIOLATION)
                        {
                            // Try to tell the user who quoted it the first time
                            if let Ok(quote) = db::queries::quotes::get_quote()
                                .bind(client, &quote.id.0)
                                .one()
                                .await
                            {
                                if let Some(id) = quote.quoted_by.to_u64() {
                                    if let Some(user_from) = get_username(client, id).await {
                                        format!("Quote Already exists. {user_from} beat you to it.")
                                    } else {
                                        error!("Failed to get username for user: {id}");
                                        String::from("Quote Already Exists")
                                    }
                                } else {
                                    error!(
                                        "Failed to convert decimal to u64: {:?}",
                                        quote.quoted_by
                                    );
                                    String::from("Quote Already Exists")
                                }
                            } else {
                                String::from("Quote Already Exists")
                            }
                        } else {
                            error!("Failed to add quote with error: {err:?}");
                            String::from("Failed to add quote. Sorry")
                        }
                    } else if res.is_ok_and(|num| num != 0) {
                        String::from("Quote Added")
                    } else {
                        String::from("Not a valid message type to quote")
                    }
                }
            } else {
                String::from("Cant quote messages from a group or channel.")
            }
        } else {
            String::from("Channels cant quote messages")
        }
    } else {
        // warn!("No reply found for msg: {msg:#?}");
        String::from("Must be used as a reply to a message to quote.")
    };
    bot.send_message(msg.chat.id, text)
        .reply_to(msg)
        .await
        .map(|_| ())
}

async fn search_quote(
    client: &Object,
    msg: Message,
    bot: Bot,
) -> Result<(), teloxide::RequestError> {
    let terms = msg.text().unwrap().split_whitespace().skip(1).collect_vec();
    if terms.is_empty() {
        // Searching by @ currently doesnt work due to limitations with telegram
        // There are some unpleasant workarounds, we'll try those later
        return bot
            .send_message(msg.chat.id, "Must provide text to search by")
            .reply_to(msg)
            .await
            .map(|_| ());
    } else if terms.len() == 1 && terms[0].starts_with('@') {
        if let Some(user) = msg.mentioned_users().find(|user| !user.is_bot) {
            let result = match db::queries::quotes::quote_from_user()
                .bind(
                    client,
                    &msg.chat.id.0,
                    &Decimal::from_u64(user.id.0).expect("Failed to convert u64 to Decimal"),
                )
                .one()
                .await
            {
                Ok(quote) => send_quote(client, bot.clone(), quote, &msg).await,
                Err(err) => Err(QuoteError::from(err)),
            };
            return match result {
                Ok(_) => Ok(()),
                Err(err) => match err {
                    QuoteError::TeloxideRequest(request_error) => {
                        error!(
                            "Failed to send a quote to the user with teloxide error: {request_error:?}"
                        );
                        Err(request_error)
                    }
                    QuoteError::DatabaseNoResults(_) => bot
                        .send_message(msg.chat.id, String::from("No Quotes in this Chat yet."))
                        .reply_to(msg.clone())
                        .await
                        .map(|_| ()),
                    err => {
                        error!("Failed to get random quote with error: {err:?}");
                        bot.send_message(
                            msg.chat.id,
                            String::from("Failed to get random quote. Sorry"),
                        )
                        .reply_to(msg.clone())
                        .await
                        .map(|_| ())
                    }
                },
            };
        }
    } else if terms[0].starts_with('@') {
        if let Some(user) = msg.mentioned_users().find(|user| !user.is_bot) {
            let result = match db::queries::quotes::search_quote_from_user()
                .bind(
                    client,
                    &msg.chat.id.0,
                    &Decimal::from_u64(user.id.0).expect("Failed to convert u64 to Decimal"),
                    &terms.iter().skip(1).join(" & "),
                )
                .one()
                .await
            {
                Ok(quote) => send_quote(client, bot.clone(), quote, &msg).await,
                Err(err) => Err(QuoteError::from(err)),
            };
            return match result {
                Ok(_) => Ok(()),
                Err(err) => match err {
                    QuoteError::TeloxideRequest(request_error) => {
                        error!(
                            "Failed to send a quote to the user with teloxide error: {request_error:?}"
                        );
                        Err(request_error)
                    }
                    QuoteError::DatabaseNoResults(_) => bot
                        .send_message(msg.chat.id, String::from("No Quotes in this Chat yet."))
                        .reply_to(msg.clone())
                        .await
                        .map(|_| ()),
                    err => {
                        error!("Failed to get random quote with error: {err:?}");
                        bot.send_message(
                            msg.chat.id,
                            String::from("Failed to get random quote. Sorry"),
                        )
                        .reply_to(msg.clone())
                        .await
                        .map(|_| ())
                    }
                },
            };
        }
    }

    let result = match db::queries::quotes::search_quote()
        .bind(client, &msg.chat.id.0, &terms.join(" & "))
        .one()
        .await
    {
        Ok(quote) => send_quote(client, bot.clone(), quote, &msg).await,
        Err(err) => Err(QuoteError::from(err)),
    };
    match result {
        Ok(_) => Ok(()),
        Err(err) => match err {
            QuoteError::TeloxideRequest(request_error) => {
                error!("Failed to send a quote to the user with teloxide error: {request_error:?}");
                Err(request_error)
            }
            QuoteError::DatabaseNoResults(_) => bot
                .send_message(msg.chat.id, String::from("No Quotes in this Chat yet."))
                .reply_to(msg.clone())
                .await
                .map(|_| ()),
            err => {
                error!("Failed to get random quote with error: {err:?}");
                bot.send_message(
                    msg.chat.id,
                    String::from("Failed to get random quote. Sorry"),
                )
                .reply_to(msg.clone())
                .await
                .map(|_| ())
            }
        },
    }
}

async fn get_quote_statistics(
    client: &Object,
    msg: Message,
    bot: Bot,
) -> Result<(), teloxide::RequestError> {
    let chat_id = msg.chat.id.0;
    let total_count = match db::queries::quotes::number_of_quotes()
        .bind(client, &chat_id)
        .one()
        .await
    {
        Ok(count) => count,
        Err(err) => {
            error!("Failed to get total quote count with err: {err:?}");
            return bot
                .send_message(msg.chat.id, "Failed to get quote rankings")
                .reply_to(msg)
                .await
                .map(|_| ());
        }
    };
    if total_count == 0 {
        return bot
            .send_message(msg.chat.id, "0 Quoted Messages in this Chat")
            .reply_to(msg)
            .await
            .map(|_| ());
    }
    let most_quoted = match db::queries::quotes::most_quoted()
        .bind(client, &chat_id)
        .all()
        .await
    {
        Ok(most_quoted) => most_quoted,
        Err(err) => {
            error!("Failed to get total quote count with err: {err:?}");
            return bot
                .send_message(msg.chat.id, "Failed to get quote rankings")
                .reply_to(msg)
                .await
                .map(|_| ());
        }
    };
    let quoted_by = match db::queries::quotes::most_quoted_by()
        .bind(client, &chat_id)
        .all()
        .await
    {
        Ok(quoted_by) => quoted_by,
        Err(err) => {
            error!("Failed to get total quote count with err: {err:?}");
            return bot
                .send_message(msg.chat.id, "Failed to get quote rankings")
                .reply_to(msg)
                .await
                .map(|_| ());
        }
    };
    let mut text = format!(
        "<b>Overall:</b>\n • {total_count} Total Quotes\n\n<b>Users With the Most Quotes:</b>\n"
    );
    for val in most_quoted {
        let username = get_username(
            client,
            val.user_from
                .to_u64()
                .expect("Failed to convert from Decimal to u64"),
        )
        .await
        .unwrap_or_else(|| val.user_from.to_string());
        let percentage =
            ((((val.count as f64) / (total_count as f64)) * 1000.) as u64) as f64 / 10.;
        text.push_str(&format!(" • {} ({percentage}%): {username}\n", val.count));
    }
    text.push_str("\n<b>Users Who Add the Most Quotes:</b>\n");
    for val in quoted_by {
        let username = get_username(
            client,
            val.quoted_by
                .to_u64()
                .expect("Failed to convert from Decimal to u64"),
        )
        .await
        .unwrap_or_else(|| val.quoted_by.to_string());
        let percentage =
            ((((val.count as f64) / (total_count as f64)) * 1000.) as u64) as f64 / 10.;
        text.push_str(&format!(" • {} ({percentage}%): {username}\n", val.count));
    }
    bot.send_message(msg.chat.id, text)
        .reply_to(msg)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await
        .map(|_| ())
}

async fn get_random_quote(
    client: &Object,
    msg: Message,
    bot: Bot,
) -> Result<(), teloxide::RequestError> {
    let result = match db::queries::quotes::random_quote()
        .bind(client, &msg.chat.id.0)
        .one()
        .await
    {
        Ok(quote) => send_quote(client, bot.clone(), quote, &msg).await,
        Err(err) => Err(QuoteError::from(err)),
    };
    match result {
        Ok(_) => Ok(()),
        Err(err) => match err {
            QuoteError::TeloxideRequest(request_error) => {
                error!("Failed to send a quote to the user with teloxide error: {request_error:?}");
                Err(request_error)
            }
            QuoteError::DatabaseNoResults(_) => bot
                .send_message(msg.chat.id, String::from("No Quotes in this Chat yet."))
                .reply_to(msg.clone())
                .await
                .map(|_| ()),
            err => {
                error!("Failed to get random quote with error: {err:?}");
                bot.send_message(
                    msg.chat.id,
                    String::from("Failed to get random quote. Sorry"),
                )
                .reply_to(msg.clone())
                .await
                .map(|_| ())
            }
        },
    }
}

pub async fn send_quote(
    client: &Object,
    bot: Bot,
    quote: Quote,
    msg: &Message,
) -> Result<(), QuoteError> {
    match quote.msg_type {
        QuoteType::Text => {
            let username = get_username(
                client,
                quote
                    .user_from
                    .to_u64()
                    .expect("Failed to convert from Decimal to u64"),
            )
            .await
            .unwrap_or_else(|| {
                let val = quote.user_from.to_string();
                warn!("Failed to get_username for user_from: {val}");
                val
            });
            let text = format!(
                "\"{}\" -{username}\n{}",
                &quote.text.expect("Text messages should have text content"),
                quote.msg_date
            );
            bot.send_message(msg.chat.id, text)
                .reply_to(msg)
                .await
                .map(|_| ())
                .map_err(|err| err.into())
        }
        QuoteType::Document => {
            let username = get_username(
                client,
                quote
                    .user_from
                    .to_u64()
                    .expect("Failed to convert from Decimal to u64"),
            )
            .await
            .unwrap_or_else(|| {
                let val = quote.user_from.to_string();
                warn!("Failed to get_username for user_from: {val}");
                val
            });
            let text = if let Some(caption) = quote.text {
                format!("\"{caption}\" -{username}\n{}", quote.msg_date)
            } else {
                format!("-{username}\n{}", quote.msg_date)
            };
            bot.send_document(
                msg.chat.id,
                InputFile::file_id(
                    quote
                        .file_id
                        .ok_or(QuoteError::TGMissingFileID(Box::new(msg.clone())))?,
                ),
            )
            .caption(text)
            .reply_to(msg)
            .await
            .map(|_| ())
            .map_err(|err| err.into())
        }
        QuoteType::Photo => {
            let username = get_username(
                client,
                quote
                    .user_from
                    .to_u64()
                    .expect("Failed to convert from Decimal to u64"),
            )
            .await
            .unwrap_or_else(|| {
                let val = quote.user_from.to_string();
                warn!("Failed to get_username for user_from: {val}");
                val
            });
            let text = if let Some(caption) = quote.text {
                format!("\"{caption}\" -{username}\n{}", quote.msg_date)
            } else {
                format!("-{username}\n{}", quote.msg_date)
            };
            bot.send_photo(
                msg.chat.id,
                InputFile::file_id(
                    quote
                        .file_id
                        .ok_or(QuoteError::TGMissingFileID(Box::new(msg.clone())))?,
                ),
            )
            .caption(text)
            .reply_to(msg)
            .has_spoiler(quote.has_spoiler)
            .await
            .map(|_| ())
            .map_err(|err| err.into())
        }
        QuoteType::Video => {
            let username = get_username(
                client,
                quote
                    .user_from
                    .to_u64()
                    .expect("Failed to convert from Decimal to u64"),
            )
            .await
            .unwrap_or_else(|| {
                let val = quote.user_from.to_string();
                warn!("Failed to get_username for user_from: {val}");
                val
            });
            let text = if let Some(caption) = quote.text {
                format!("\"{caption}\" -{username}\n{}", quote.msg_date)
            } else {
                format!("-{username}\n{}", quote.msg_date)
            };
            bot.send_video(
                msg.chat.id,
                InputFile::file_id(
                    quote
                        .file_id
                        .ok_or(QuoteError::TGMissingFileID(Box::new(msg.clone())))?,
                ),
            )
            .caption(text)
            .reply_to(msg)
            .has_spoiler(quote.has_spoiler)
            .await
            .map(|_| ())
            .map_err(|err| err.into())
        }
        QuoteType::Voice => {
            let username = get_username(
                client,
                quote
                    .user_from
                    .to_u64()
                    .expect("Failed to convert from Decimal to u64"),
            )
            .await
            .unwrap_or_else(|| {
                let val = quote.user_from.to_string();
                warn!("Failed to get_username for user_from: {val}");
                val
            });
            let text = if let Some(caption) = quote.text {
                format!("\"{caption}\" -{username}\n{}", quote.msg_date)
            } else {
                format!("-{username}\n{}", quote.msg_date)
            };
            bot.send_voice(
                msg.chat.id,
                InputFile::file_id(
                    quote
                        .file_id
                        .ok_or(QuoteError::TGMissingFileID(Box::new(msg.clone())))?,
                ),
            )
            .caption(text)
            .reply_to(msg)
            .await
            .map(|_| ())
            .map_err(|err| err.into())
        }
    }
}

pub async fn add_quote(
    client: &Object,
    quote: &Message,
    quoter: &User,
) -> Result<u64, db::tokio_postgres::Error> {
    if !matches!(quote.kind, MessageKind::Common(_)) {
        return Ok(0);
    }
    let (file_id, text, has_spoiler, msg_type) = if let Some(photo) = quote.photo() {
        assert!(!photo.is_empty());
        let photo = photo
            .iter()
            .sorted_unstable_by_key(|p| p.width * p.height)
            .next_back()
            .unwrap();
        (
            Some(photo.file.id.clone()),
            quote.caption(),
            quote.has_media_spoiler(),
            QuoteType::Photo,
        )
    } else if let Some(voice) = quote.voice() {
        (
            Some(voice.file.id.clone()),
            quote.caption(),
            quote.has_media_spoiler(),
            QuoteType::Voice,
        )
    } else if let Some(video) = quote.video() {
        (
            Some(video.file.id.clone()),
            quote.caption(),
            quote.has_media_spoiler(),
            QuoteType::Video,
        )
    } else if let Some(document) = quote.document() {
        (
            Some(document.file.id.clone()),
            quote.caption(),
            quote.has_media_spoiler(),
            QuoteType::Document,
        )
    } else if quote.video_note().is_some()
        || quote.game().is_some()
        || quote.poll().is_some()
        || quote.animation().is_some()
    {
        return Ok(0);
    } else {
        (None, quote.text(), false, QuoteType::Text)
    };
    let quote_user = quote.from.clone().unwrap();
    crate::utils::set_username(
        client,
        quote_user.id.0,
        quote_user
            .username
            .unwrap_or_else(|| quoter.full_name().clone()),
    )
    .await;
    crate::utils::set_username(
        client,
        quoter.id.0,
        quoter
            .username
            .clone()
            .unwrap_or_else(|| quoter.full_name().clone()),
    )
    .await;
    db::queries::quotes::add_quote()
        .bind(
            client,
            &quote.id.0,
            &Decimal::from_u64(quote_user.id.0).expect("Failed to convert from u64 to decimal"),
            &quote.chat.id.0,
            &Decimal::from_u64(quoter.id.0).expect("Failed to convert from u64 to decimal"),
            &msg_type,
            &quote.date.date_naive(),
            &has_spoiler,
            &text,
            &file_id,
        )
        .await
}
