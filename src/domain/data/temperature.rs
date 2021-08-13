use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Temperature {
    pub value: f32,
    pub id: i32,
    pub measured_at: NaiveDateTime,
    pub localisation_id: i32,
}

#[derive(Deserialize)]
pub struct UserTemperature {
    pub value: i32,
    pub measured_at: NaiveDateTime,
    pub localisation_id: i32,
}
