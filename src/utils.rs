use cached::proc_macro::cached;
use db::deadpool_postgres::Object;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use tracing::error;

pub async fn set_username(client: &Object, user_id: u64, username: String) {
    match client
        .query(
            "SELECT merge_username($1, $2)",
            &[
                &Decimal::from_u64(user_id).expect("Failed to convert u64 to Decimal"),
                &username,
            ],
        )
        .await
    {
        Ok(_) => {}
        Err(err) => {
            error!("Error updating username, may not be a problem, review later: {err:?}");
        }
    }
}

pub async fn is_user_opt_out(client: &Object, user_id: u64) -> bool {
    db::queries::user_management::is_user_opt_out()
        .bind(
            client,
            &Decimal::from_u64(user_id).expect("Failed to convert u64 to Decimal"),
        )
        .one()
        .await
        .is_ok()
}

#[cached(
    sync_writes = "by_key",
    key = "u64",
    convert = r#"{ user_id }"#,
    time = 3600,
    option = true
)]
pub async fn get_username(client: &Object, user_id: u64) -> Option<String> {
    db::queries::user_management::get_name()
        .bind(
            client,
            &Decimal::from_u64(user_id).expect("Failed to convert u64 to Decimal"),
        )
        .one()
        .await
        .ok()
}
