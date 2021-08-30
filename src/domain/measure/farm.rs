use super::rack;

use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct Farm {
    id: i32,
    plant: PlantType,
    racks: Vec<rack::Rack>,
    name: String,
}

#[derive(Deserialize)]
pub struct InputFarm {
    plant: PlantType,
    racks: Vec<rack::Rack>,
    name: String,
}

#[derive(Clone, Copy, Serialize, Debug, Deserialize)]
pub enum PlantType {
    Tomato,
    Eggplant,
    Strawberry,
    Cherry,
    Salad,
    NewSpecies,
}
