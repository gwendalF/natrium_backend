use super::rack;
use crate::{data::rack::Rack, errors::Result};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Farm {
    id: i64,
    plant: PlantType,
    racks: Vec<rack::Rack>,
    name: String,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum PlantType {
    Tomato,
    Eggplant,
    Strawberry,
    Cherry,
    Salad,
    NewSpecies,
}

impl Farm {
    pub async fn get_all(pool: &PgPool, user_id: i64) -> Result<Vec<Farm>> {
        let db_farms = sqlx::query!(
            "SELECT farm.id, name, species FROM farm 
            INNER JOIN plant ON plant.id=farm.plant_id 
            WHERE hub_id=$1",
            hub_id
        )
        .fetch_all(pool)
        .await?;
        match db_farms.len() {
            0 => Ok(vec![]),
            _ => {
                let mut farms = Vec::with_capacity(db_farms.len());
                for farm in db_farms {
                    let plant = match farm.species.as_ref() {
                        "Tomato" => PlantType::Tomato,
                        "Salad" => PlantType::Salad,
                        "Eggplant" => PlantType::Eggplant,
                        "Strawberry" => PlantType::Strawberry,
                        _ => PlantType::NewSpecies,
                    };
                    farms.push(Farm {
                        id: farm.id,
                        plant,
                        name: farm.name,
                        racks: Rack::get_all(pool, farm.id, plant).await?,
                    });
                }
                Ok(farms)
            }
        }
    }
}
