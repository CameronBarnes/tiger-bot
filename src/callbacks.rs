use db::{
    deadpool_postgres::{Object, Pool},
    tokio_postgres,
};
use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use teloxide::{Bot, prelude::Requester, sugar::bot::BotMessagesExt, types::CallbackQuery};
use tracing::{debug, error, warn};

pub async fn endpoint(
    db_pool: Pool,
    mut cache_pool: ConnectionManager,
    bot: Bot,
    callback: CallbackQuery,
) -> Result<(), teloxide::RequestError> {
    debug!("Callback!");
    let mut client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to get DB client with err: {err:?}");
            return Ok(());
        }
    };

    bot.answer_callback_query(callback.id.clone()).await?;

    let Some(data) = callback.data.clone() else {
        warn!("No callback data");
        return Ok(());
    };

    let Some((tag, id)) = data.split_once("||") else {
        warn!("Failed to parse callback data: {data}");
        return Ok(());
    };

    let Some(message) = callback.regular_message() else {
        warn!("Failed to get original message for callback: {callback:?}");
        return Ok(());
    };

    let user_from = callback.from.id.0;
    let Ok(cached_from): Result<u64, RedisError> = cache_pool.get(id).await else {
        error!("Failed to get cached id for callback: {callback:?}");
        return bot.send_message(message.chat.id, "Failed to get cached user data, sorry, please try running the command again later.").await.map(|_| ());
    };

    // This is to make sure we only opt out the person who's asking for it and not just anyone who
    // clicks the button
    if user_from != cached_from {
        debug!(
            "Callback button was not pressed by the expected user, ignoring.\nExpected: {cached_from}, From: {user_from}"
        );
        return Ok(());
    }

    debug!("Callback good, processing!");
    match tag {
        "opt_in_yes" => {
            let result = opt_in(&client, callback.from.id.0).await;

            if result.is_ok() {
                let mut request = bot.edit_reply_markup(message);
                request.reply_markup = None;
                request.await?;

                debug!("Callback success, opt in yes");
                bot.edit_text(message, "You have successfully opted in.")
                    .await
                    .map(|_| ())
            } else {
                error!("Failed to opt in with error: {result:?}");
                Ok(())
            }
        }
        "opt_out_yes" => {
            let result = opt_out(&mut client, callback.from.id.0).await;

            if result.is_ok() {
                let mut request = bot.edit_reply_markup(message);
                request.reply_markup = None;
                request.await?;

                debug!("Callback success, opt out yes");
                bot.edit_text(message, "You have successfully opted out.")
                    .await
                    .map(|_| ())
            } else {
                error!("Failed to opt out with error: {result:?}");
                Ok(())
            }
        }
        "opt_in_no" => {
            let mut request = bot.edit_reply_markup(message);
            request.reply_markup = None;
            request.await?;

            debug!("Callback success, opt in cancel");
            bot.edit_text(message, "Opt in canceled").await.map(|_| ())
        }
        "opt_out_no" => {
            let mut request = bot.edit_reply_markup(message);
            request.reply_markup = None;
            request.await?;

            debug!("Callback success, opt out cancel");
            bot.edit_text(message, "Opt out canceled").await.map(|_| ())
        }
        _ => {
            warn!("Unknown tag value: {tag} with data: {data}");
            Ok(())
        }
    }
}

async fn opt_out(client: &mut Object, user_id: u64) -> Result<(), tokio_postgres::Error> {
    let transaction = client.transaction().await?;
    let user_id = &Decimal::from_u64(user_id).expect("Failed to convert u64 to Decimal");
    db::queries::user_management::add_opt_out_user()
        .bind(&transaction, user_id)
        .await?;
    db::queries::quotes::purge_quotes_for_privacy()
        .bind(&transaction, user_id)
        .await?;
    db::queries::user_management::remove_name()
        .bind(&transaction, user_id)
        .await?;

    transaction.commit().await
}

async fn opt_in(client: &Object, user_id: u64) -> Result<u64, tokio_postgres::Error> {
    db::queries::user_management::remove_opt_out_user()
        .bind(
            client,
            &Decimal::from_u64(user_id).expect("Failed to convert u64 to Decimal"),
        )
        .await
}
