use crate::Result;
use sqlx::PgPool;

pub async fn check_existing_user(pool: &PgPool, google_subject: &str) -> Result<Option<i32>> {
    if let Some(record) = sqlx::query!(
        "SELECT user_id FROM provider_user_mapper WHERE subject=$1",
        google_subject
    )
    .fetch_optional(pool)
    .await?
    {
        Ok(Some(record.user_id))
    } else {
        Ok(None)
    }
}
