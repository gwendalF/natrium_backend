use super::farm::PlantType;
use crate::errors::Result;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize, Debug)]
pub struct Rack {
    pub id: i64,
    pub name: String,
    pub localisation_id: i64,
}

#[derive(Serialize, Debug)]
pub struct UserRack {
    pub name: String,
    pub localisation_id: i64,
}
impl Rack {
    pub async fn get_all(
        pool: &PgPool,
        user_id: i64,
        plant_type: PlantType,
    ) -> Result<Vec<UserRack>> {
        Ok(sqlx::query_as!(
            UserRack,
            "SELECT name, localisation_id FROM rack 
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
