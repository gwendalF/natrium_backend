use chrono::NaiveDateTime;

pub struct Temperature {
    pub id: i64,
    pub temperature: f32,
    pub measured_at: NaiveDateTime,
    pub measurer_id: i64,
}
