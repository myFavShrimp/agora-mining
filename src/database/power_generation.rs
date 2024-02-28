use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::PrimitiveDateTime;

use super::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerGeneration {
    pub date_id: PrimitiveDateTime,
    pub biomass: Option<f64>,
    pub hard_coal: Option<f64>,
    pub hydro: Option<f64>,
    pub lignite: Option<f64>,
    pub natural_gas: Option<f64>,
    pub nuclear: Option<f64>,
    pub other: Option<f64>,
    pub pumped_storage_generation: Option<f64>,
    pub solar: Option<f64>,
    pub total_conventional_power_plant: Option<f64>,
    pub wind_offshore: Option<f64>,
    pub wind_onshore: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Fields {
    #[serde(rename = "Biomass")]
    Biomass,
    #[serde(rename = "Hard Coal")]
    HardCoal,
    #[serde(rename = "Hydro")]
    Hydro,
    #[serde(rename = "Lignite")]
    Lignite,
    #[serde(rename = "Natural Gas")]
    NaturalGas,
    #[serde(rename = "Nuclear")]
    Nuclear,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Pumped storage generation")]
    PumpedStorageGeneration,
    #[serde(rename = "Solar")]
    Solar,
    #[serde(rename = "Total conventional power plant")]
    TotalConventionalPowerPlant,
    #[serde(rename = "Wind offshore")]
    WindOffshore,
    #[serde(rename = "Wind onshore")]
    WindOnshore,
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
// pub enum Fields {
//     #[serde(rename(deserialize = "Biomass", serialize = "Bioenergie"))]
//     Biomass,
//     #[serde(rename(
//         deserialize = "Grid emission factor",
//         serialize = "Netzemissionsfaktor"
//     ))]
//     GridEmissionFactor,
//     #[serde(rename(deserialize = "Hard Coal", serialize = "Steinkohle"))]
//     HardCoal,
//     #[serde(rename(deserialize = "Hydro", serialize = "Wasserkraft"))]
//     Hydro,
//     #[serde(rename(deserialize = "Lignite", serialize = "Braunkohle"))]
//     Lignite,
//     #[serde(rename(deserialize = "Natural Gas", serialize = "Erdgas"))]
//     NaturalGas,
//     #[serde(rename(deserialize = "Nuclear", serialize = "Kernkraft"))]
//     Nuclear,
//     #[serde(rename(deserialize = "Other", serialize = "Andere"))]
//     Other,
//     #[serde(rename(
//         deserialize = "Pumped storage generation",
//         serialize = "Pumpspeicherkraftwerk"
//     ))]
//     PumpedStorageGeneration,
//     #[serde(rename(deserialize = "Solar", serialize = "Solar"))]
//     Solar,
//     #[serde(rename(
//         deserialize = "Total conventional power plant",
//         serialize = "Konventionelles Kraftwerk insgesamt"
//     ))]
//     TotalConventionalPowerPlant,
//     #[serde(rename(
//         deserialize = "Total electricity demand",
//         serialize = "Gesamtstrombedarf"
//     ))]
//     TotalElectricityDemand,
//     #[serde(rename(
//         deserialize = "Total grid emissions",
//         serialize = "Netzemissionen insgesamt"
//     ))]
//     TotalGridEmissions,
//     #[serde(rename(deserialize = "Wind offshore", serialize = "Offshore-Wind"))]
//     WindOffshore,
//     #[serde(rename(deserialize = "Wind onshore", serialize = "Onshore-Wind"))]
//     WindOnshore,
// }

impl Entity<Fields> for PowerGeneration {
    fn unit() -> String {
        return "mW/h".to_string()
    }

    fn all_fields() -> Vec<Fields> {
        vec![
            Fields::Biomass,
            Fields::HardCoal,
            Fields::Hydro,
            Fields::Lignite,
            Fields::NaturalGas,
            Fields::Nuclear,
            Fields::Other,
            Fields::PumpedStorageGeneration,
            Fields::Solar,
            Fields::TotalConventionalPowerPlant,
            Fields::WindOffshore,
            Fields::WindOnshore,
        ]
    }

    fn set_by_field(&mut self, field: Fields, value: f64) {
        match field {
            Fields::Biomass => self.biomass = Some(value),
            Fields::HardCoal => self.hard_coal = Some(value),
            Fields::Hydro => self.hydro = Some(value),
            Fields::Lignite => self.lignite = Some(value),
            Fields::NaturalGas => self.natural_gas = Some(value),
            Fields::Nuclear => self.nuclear = Some(value),
            Fields::Other => self.other = Some(value),
            Fields::PumpedStorageGeneration => self.pumped_storage_generation = Some(value),
            Fields::Solar => self.solar = Some(value),
            Fields::TotalConventionalPowerPlant => {
                self.total_conventional_power_plant = Some(value)
            }
            Fields::WindOffshore => self.wind_offshore = Some(value),
            Fields::WindOnshore => self.wind_onshore = Some(value),
        }
    }

    fn set_id(&mut self, date: PrimitiveDateTime) {
        self.date_id = date
    }

    fn api_view_name() -> &'static str {
        "live_gen_plus_emi_de_hourly"
    }

    fn api_kpi_name() -> &'static str {
        "power_generation"
    }
}

impl Default for PowerGeneration {
    fn default() -> Self {
        Self {
            date_id: PrimitiveDateTime::MIN,
            biomass: Default::default(),
            hard_coal: Default::default(),
            hydro: Default::default(),
            lignite: Default::default(),
            natural_gas: Default::default(),
            nuclear: Default::default(),
            other: Default::default(),
            pumped_storage_generation: Default::default(),
            solar: Default::default(),
            total_conventional_power_plant: Default::default(),
            wind_offshore: Default::default(),
            wind_onshore: Default::default(),
        }
    }
}

impl PowerGeneration {
    pub async fn create(
        connection: &PgPool,
        value: &PowerGeneration,
    ) -> Result<PowerGeneration, sqlx::Error> {
        sqlx::query_as!(
            PowerGeneration,
            "
                INSERT INTO power_generation 
                    (date_id, biomass, hard_coal, hydro, lignite, natural_gas, nuclear, other, pumped_storage_generation, solar, total_conventional_power_plant, wind_offshore, wind_onshore)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                RETURNING *
            ",
            value.date_id,
            value.biomass,
            value.hard_coal,
            value.hydro,
            value.lignite,
            value.natural_gas,
            value.nuclear,
            value.other,
            value.pumped_storage_generation,
            value.solar,
            value.total_conventional_power_plant,
            value.wind_offshore,
            value.wind_onshore,
        ).fetch_one(connection).await
    }

    pub async fn create_many(
        connection: &PgPool,
        values: Vec<PowerGeneration>,
    ) -> Result<Vec<PowerGeneration>, sqlx::Error> {
        let mut result = Vec::new();

        for item in values {
            result.push(PowerGeneration::create(connection, &item).await?);
        }

        Ok(result)
    }

    pub async fn delete_all(connection: &PgPool) -> Result<Vec<PowerGeneration>, sqlx::Error> {
        sqlx::query_as!(
            PowerGeneration,
            "
                DELETE FROM power_generation 
                RETURNING *
            ",
        )
        .fetch_all(connection)
        .await
    }

    pub async fn find_all_ordered_by_date(
        connection: &PgPool,
    ) -> Result<Vec<PowerGeneration>, sqlx::Error> {
        sqlx::query_as!(
            PowerGeneration,
            "
                SELECT * FROM power_generation ORDER BY date_id ASC
            ",
        )
        .fetch_all(connection)
        .await
    }
}
