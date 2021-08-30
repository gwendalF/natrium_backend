use crate::Result;
use sqlx::PgPool;

pub async fn check_existing_user(pool: &PgPool, subject: &str) -> Result<Option<i32>> {
    if let Some(record) = sqlx::query!(
        "SELECT user_id FROM provider_user_mapper WHERE subject=$1",
        subject
    )
    .fetch_optional(pool)
    .await?
    {
        Ok(Some(record.user_id))
    } else {
        Ok(None)
    }
}

pub async fn get_user_id(subject: &str, pool: &PgPool) -> Result<i32> {
    Ok(sqlx::query!(
        "SELECT user_account.id FROM user_account JOIN provider_user_mapper ON subject=$1",
        subject,
    )
    .fetch_one(pool)
    .await?
    .id)
}
