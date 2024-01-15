use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::PrimitiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerGenerationAndConsumption {
    pub date_id: PrimitiveDateTime,
    pub biomass: Option<f64>,
    pub grid_emission_factor: Option<f64>,
    pub hard_coal: Option<f64>,
    pub hydro: Option<f64>,
    pub lignite: Option<f64>,
    pub natural_gas: Option<f64>,
    pub nuclear: Option<f64>,
    pub other: Option<f64>,
    pub pumped_storage_generation: Option<f64>,
    pub solar: Option<f64>,
    pub total_conventional_power_plant: Option<f64>,
    pub total_electricity_demand: Option<f64>,
    pub total_grid_emissions: Option<f64>,
    pub wind_offshore: Option<f64>,
    pub wind_onshore: Option<f64>,
}

impl Default for PowerGenerationAndConsumption {
    fn default() -> Self {
        Self {
            date_id: PrimitiveDateTime::MIN,
            biomass: Default::default(),
            grid_emission_factor: Default::default(),
            hard_coal: Default::default(),
            hydro: Default::default(),
            lignite: Default::default(),
            natural_gas: Default::default(),
            nuclear: Default::default(),
            other: Default::default(),
            pumped_storage_generation: Default::default(),
            solar: Default::default(),
            total_conventional_power_plant: Default::default(),
            total_electricity_demand: Default::default(),
            total_grid_emissions: Default::default(),
            wind_offshore: Default::default(),
            wind_onshore: Default::default(),
        }
    }
}

impl PowerGenerationAndConsumption {
    pub async fn create(
        connection: &PgPool,
        value: &PowerGenerationAndConsumption,
    ) -> Result<PowerGenerationAndConsumption, sqlx::Error> {
        sqlx::query_as!(
            PowerGenerationAndConsumption,
            "
                INSERT INTO power_generation_and_consumption 
                    (date_id, biomass, grid_emission_factor, hard_coal, hydro, lignite, natural_gas, nuclear, other, pumped_storage_generation, solar, total_conventional_power_plant, total_electricity_demand, total_grid_emissions, wind_offshore, wind_onshore)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING *
            ",
            value.date_id,
            value.biomass,
            value.grid_emission_factor,
            value.hard_coal,
            value.hydro,
            value.lignite,
            value.natural_gas,
            value.nuclear,
            value.other,
            value.pumped_storage_generation,
            value.solar,
            value.total_conventional_power_plant,
            value.total_electricity_demand,
            value.total_grid_emissions,
            value.wind_offshore,
            value.wind_onshore,
        ).fetch_one(connection).await
    }

    pub async fn create_many(
        connection: &PgPool,
        values: Vec<PowerGenerationAndConsumption>,
    ) -> Result<Vec<PowerGenerationAndConsumption>, sqlx::Error> {
        let mut result = Vec::new();

        for item in values {
            result.push(dbg!(
                PowerGenerationAndConsumption::create(connection, &item).await
            )?);
        }

        Ok(result)
    }
}
