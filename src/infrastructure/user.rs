use crate::errors::Result;
use sqlx::PgPool;

pub async fn check_existing_user(
    provider_subject: &str,
    database_pool: &PgPool,
) -> Result<Option<i32>> {
    if let Some(record) = sqlx::query!(
        "SELECT user_id FROM provider_user_mapper WHERE subject=$1",
        provider_subject
    )
    .fetch_optional(database_pool)
    .await?
    {
        Ok(Some(record.user_id))
    } else {
        Ok(None)
    }
}
