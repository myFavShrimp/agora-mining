use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{Date, PrimitiveDateTime};

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
    fn chart_display_name(field: &Fields) -> &'static str {
        match field {
            Fields::Biomass => "Bioenergie",
            Fields::HardCoal => "Steinkohle",
            Fields::Hydro => "Wasserkraft",
            Fields::Lignite => "Braunkohle",
            Fields::NaturalGas => "Erdgas",
            Fields::Nuclear => "Kernkraft",
            Fields::Other => "Andere",
            Fields::PumpedStorageGeneration => "Pumpspeicherkraftwerk",
            Fields::Solar => "Solar",
            Fields::TotalConventionalPowerPlant => "Konventionelles Kraftwerk insgesamt",
            Fields::WindOffshore => "Offshore-Wind",
            Fields::WindOnshore => "Onshore-Wind",
        }
    }

    fn id(&self) -> PrimitiveDateTime {
        self.date_id
    }

    fn unit(_field: &Fields) -> String {
        "mW/h".to_string()
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

    fn set_by_field(&mut self, field: &Fields, value: Option<f64>) {
        match field {
            Fields::Biomass => self.biomass = value,
            Fields::HardCoal => self.hard_coal = value,
            Fields::Hydro => self.hydro = value,
            Fields::Lignite => self.lignite = value,
            Fields::NaturalGas => self.natural_gas = value,
            Fields::Nuclear => self.nuclear = value,
            Fields::Other => self.other = value,
            Fields::PumpedStorageGeneration => self.pumped_storage_generation = value,
            Fields::Solar => self.solar = value,
            Fields::TotalConventionalPowerPlant => self.total_conventional_power_plant = value,
            Fields::WindOffshore => self.wind_offshore = value,
            Fields::WindOnshore => self.wind_onshore = value,
        }
    }

    fn get_by_field(&self, field: &Fields) -> Option<f64> {
        match field {
            Fields::Biomass => self.biomass,
            Fields::HardCoal => self.hard_coal,
            Fields::Hydro => self.hydro,
            Fields::Lignite => self.lignite,
            Fields::NaturalGas => self.natural_gas,
            Fields::Nuclear => self.nuclear,
            Fields::Other => self.other,
            Fields::PumpedStorageGeneration => self.pumped_storage_generation,
            Fields::Solar => self.solar,
            Fields::TotalConventionalPowerPlant => self.total_conventional_power_plant,
            Fields::WindOffshore => self.wind_offshore,
            Fields::WindOnshore => self.wind_onshore,
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

    fn api_filter_values_key() -> &'static str {
        "generation"
    }

    async fn create(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
        ).fetch_one(&mut **connection).await
    }

    async fn create_many(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        values: Vec<PowerGeneration>,
    ) -> Result<Vec<PowerGeneration>, sqlx::Error> {
        let mut result = Vec::new();

        for item in values {
            result.push(PowerGeneration::create(connection, &item).await?);
        }

        Ok(result)
    }

    async fn delete_all(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Vec<PowerGeneration>, sqlx::Error> {
        sqlx::query_as!(
            PowerGeneration,
            "
                DELETE FROM power_generation 
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
    ) -> Result<Vec<PowerGeneration>, sqlx::Error> {
        sqlx::query_as!(
            PowerGeneration,
            "
                SELECT * FROM power_generation WHERE date_id >= $1 AND date_id <= $2 ORDER BY date_id ASC
            ",
            from.midnight(),
            to.midnight(), // TODO: last minute of day
        )
        .fetch_all(connection)
        .await
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
