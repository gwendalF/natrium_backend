use super::farm;
use serde::Serialize;


#[derive(Serialize)]
pub struct Hub {
    id: i32,
    name: String,
    farms: Vec<farm::Farm>,
}
