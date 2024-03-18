use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{Date, PrimitiveDateTime};

use super::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerEmission {
    pub date_id: PrimitiveDateTime,
    pub hard_coal: Option<f64>,
    pub lignite: Option<f64>,
    pub natural_gas: Option<f64>,
    pub other: Option<f64>,
    pub total_grid_emissions: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Fields {
    #[serde(rename = "Hard Coal")]
    HardCoal,
    #[serde(rename = "Lignite")]
    Lignite,
    #[serde(rename = "Natural Gas")]
    NaturalGas,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Total grid emissions")]
    TotalGridEmissions,
}

impl Entity<Fields> for PowerEmission {
    fn unit(_field: &Fields) -> String {
        "tCO₂/h".to_string()
    }

    fn all_fields() -> Vec<Fields> {
        vec![
            Fields::HardCoal,
            Fields::Lignite,
            Fields::NaturalGas,
            Fields::Other,
            Fields::TotalGridEmissions,
        ]
    }

    fn set_by_field(&mut self, field: &Fields, value: Option<f64>) {
        match field {
            Fields::HardCoal => self.hard_coal = value,
            Fields::Lignite => self.lignite = value,
            Fields::NaturalGas => self.natural_gas = value,
            Fields::Other => self.other = value,
            Fields::TotalGridEmissions => self.total_grid_emissions = value,
        }
    }

    fn get_by_field(&self, field: &Fields) -> Option<f64> {
        match field {
            Fields::HardCoal => self.hard_coal,
            Fields::Lignite => self.lignite,
            Fields::NaturalGas => self.natural_gas,
            Fields::Other => self.other,
            Fields::TotalGridEmissions => self.total_grid_emissions,
        }
    }

    fn set_id(&mut self, date: PrimitiveDateTime) {
        self.date_id = date
    }

    fn api_view_name() -> &'static str {
        "live_emi_by_fuel_de_hourly"
    }

    fn api_kpi_name() -> &'static str {
        "power_emission"
    }

    fn api_filter_values_key() -> &'static str {
        "generation"
    }

    async fn create(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        value: &PowerEmission,
    ) -> Result<PowerEmission, sqlx::Error> {
        sqlx::query_as!(
            PowerEmission,
            "
                INSERT INTO power_emission 
                    (date_id, hard_coal, lignite, natural_gas, other, total_grid_emissions)
                    VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
            ",
            value.date_id,
            value.hard_coal,
            value.lignite,
            value.natural_gas,
            value.other,
            value.total_grid_emissions,
        )
        .fetch_one(&mut **connection)
        .await
    }

    async fn create_many(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        values: Vec<PowerEmission>,
    ) -> Result<Vec<PowerEmission>, sqlx::Error> {
        let mut result = Vec::new();

        for item in values {
            result.push(PowerEmission::create(connection, &item).await?);
        }

        Ok(result)
    }

    async fn delete_all(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Vec<PowerEmission>, sqlx::Error> {
        sqlx::query_as!(
            PowerEmission,
            "
                DELETE FROM power_emission 
                RETURNING *
            ",
        )
        .fetch_all(&mut **connection)
        .await
    }

    async fn find_all_ordered_by_date(
        connection: &PgPool,
        from: &Date,
        to: &Date,
    ) -> Result<Vec<PowerEmission>, sqlx::Error> {
        sqlx::query_as!(
            PowerEmission,
            "
                SELECT * FROM power_emission WHERE date_id >= $1 AND date_id <= $2 ORDER BY date_id ASC
            ",
            from.midnight(),
            to.midnight(), // TODO: last minute of day
        )
        .fetch_all(connection)
        .await
    }
}

impl Default for PowerEmission {
    fn default() -> Self {
        Self {
            date_id: PrimitiveDateTime::MIN,
            hard_coal: Default::default(),
            lignite: Default::default(),
            natural_gas: Default::default(),
            other: Default::default(),
            total_grid_emissions: Default::default(),
        }
    }
}
