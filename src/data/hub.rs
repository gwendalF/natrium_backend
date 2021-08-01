use super::farm;
use crate::errors::Result;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Hub {
    id: i32,
    name: String,
    farms: Vec<farm::Farm>,
}

impl Hub {
    pub async fn get_all(pool: &PgPool, user_id: i32) -> Result<Vec<Hub>> {
        let hub_data = sqlx::query!(
            "SELECT hub.id, name FROM hub 
            INNER JOIN localisation 
            ON localisation.id=hub.localisation_id AND localisation.user_id=$1",
            user_id
        )
        .fetch_all(pool)
        .await?;
        match hub_data.len() {
            0 => Ok(vec![]),
            _ => {
                let mut hubs = Vec::with_capacity(hub_data.len());
                for db_hub in hub_data {
                    let farms = farm::Farm::get_all(pool, db_hub.id).await?;
                    hubs.push(Hub {
                        id: db_hub.id,
                        name: db_hub.name,
                        farms,
                    });
                }
                Ok(hubs)
            }
        }
    }
}
