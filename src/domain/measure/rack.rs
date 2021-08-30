
use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug, Deserialize)]
pub struct Rack {
    pub id: i32,
    pub name: String,
    pub localisation_id: i32,
}

#[derive(Serialize, Debug)]
pub struct InputRack {
    pub name: String,
    pub localisation_id: i32,
}
