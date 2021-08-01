use crate::errors::{AppError, Result};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Temperature {
    pub id: i32,
    pub temperature: f32,
    pub measured_at: NaiveDateTime,
    pub localisation_id: i32,
}

#[derive(Deserialize)]
pub struct UserTemperature {
    pub temperature: f32,
    pub measured_at: NaiveDateTime,
    pub localisation_id: i32,
}

impl Temperature {
    pub async fn get_by_rack(pool: &PgPool, localisation_id: i32) -> Result<Vec<Temperature>> {
        Ok(sqlx::query_as!(Temperature,
            "SELECT id,temperature, measured_at, localisation_id FROM temperature WHERE localisation_id=$1",
            localisation_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn add(pool: &PgPool, temperature: UserTemperature) -> Result<Temperature> {
        Ok(sqlx::query_as!(
            Temperature,
            "INSERT INTO temperature (measured_at, temperature, localisation_id) 
        VALUES ($1, $2, $3) RETURNING id, measured_at, temperature, localisation_id",
            temperature.measured_at,
            temperature.temperature,
            temperature.localisation_id
        )
        .fetch_one(pool)
        .await?)
    }
}
