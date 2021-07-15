use chrono::NaiveDateTime;

pub struct Temperature {
    pub id: i64,
    pub value: f32,
    pub at: NaiveDateTime,
    pub measurer_id: i64,
}
