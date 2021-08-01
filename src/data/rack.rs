use super::farm::PlantType;
use crate::errors::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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

impl Rack {
    pub async fn get_all(pool: &PgPool, user_id: i32, plant_type: PlantType) -> Result<Vec<Rack>> {
        Ok(sqlx::query_as!(
            Rack,
            "SELECT rack.id, name, localisation_id FROM rack 
        INNER JOIN localisation 
        ON localisation.user_id=$1 AND 
        localisation.localisation_type=$2",
            user_id,
            "Rack"
        )
        .fetch_all(pool)
        .await?)
    }
}
