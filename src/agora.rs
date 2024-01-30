use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::{
    macros::{date, format_description},
    PrimitiveDateTime,
};

use crate::database::power_generation_and_consumption::PowerGenerationAndConsumption;

#[derive(Serialize, Deserialize, Debug)]
pub enum GenerationKind {
    #[serde(rename = "Biomass")]
    Biomass,
    #[serde(rename = "Grid emission factor")]
    GridEmissionFactor,
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
    #[serde(rename = "Total electricity demand")]
    TotalElectricityDemand,
    #[serde(rename = "Total grid emissions")]
    TotalGridEmissions,
    #[serde(rename = "Wind offshore")]
    WindOffshore,
    #[serde(rename = "Wind onshore")]
    WindOnshore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgoraApiResponseData {
    pub data: Vec<(String, f64, GenerationKind)>,
    pub columns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgoraApiResponse {
    pub status: bool,
    pub data: AgoraApiResponseData,
}

impl PowerGenerationAndConsumption {
    pub fn set_by_kind(&mut self, kind: GenerationKind, value: f64) {
        match kind {
            GenerationKind::Biomass => self.biomass = Some(value),
            GenerationKind::GridEmissionFactor => self.grid_emission_factor = Some(value),
            GenerationKind::HardCoal => self.hard_coal = Some(value),
            GenerationKind::Hydro => self.hydro = Some(value),
            GenerationKind::Lignite => self.lignite = Some(value),
            GenerationKind::NaturalGas => self.natural_gas = Some(value),
            GenerationKind::Nuclear => self.nuclear = Some(value),
            GenerationKind::Other => self.other = Some(value),
            GenerationKind::PumpedStorageGeneration => self.pumped_storage_generation = Some(value),
            GenerationKind::Solar => self.solar = Some(value),
            GenerationKind::TotalConventionalPowerPlant => {
                self.total_conventional_power_plant = Some(value)
            }
            GenerationKind::TotalElectricityDemand => self.total_electricity_demand = Some(value),
            GenerationKind::TotalGridEmissions => self.total_grid_emissions = Some(value),
            GenerationKind::WindOffshore => self.wind_offshore = Some(value),
            GenerationKind::WindOnshore => self.wind_onshore = Some(value),
        }
    }
}

impl TryInto<Vec<PowerGenerationAndConsumption>> for AgoraApiResponse {
    type Error = time::Error;

    fn try_into(self) -> Result<Vec<PowerGenerationAndConsumption>, Self::Error> {
        let data = self.data.data;

        let mut result_map: HashMap<PrimitiveDateTime, PowerGenerationAndConsumption> =
            HashMap::new();

        let format_description =
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

        for item in data {
            let (date, value, kind) = item;
            let parsed_date = PrimitiveDateTime::parse(&date, format_description)?;

            if let Some(existing_data) = result_map.get_mut(&parsed_date) {
                existing_data.set_by_kind(kind, value);
            } else {
                let mut new_data = PowerGenerationAndConsumption::default();
                new_data.date_id = parsed_date;
                new_data.set_by_kind(kind, value);
                result_map.insert(parsed_date, new_data);
            }
        }

        Ok(result_map.into_values().collect())
    }
}

static AGORA_API_URL: &str = "https://api.agora-energy.org/api/raw-data";
static AGORA_API_KEY_HEADER_NAME: &str = "api-key";
static AGORA_API_KEY_HEADER_VALUE: &str = "agora_live_62ce76dd202927.67115829";

static AGORA_API_FROM_DATE: time::Date = date!(2012 - 01 - 01);

// #[cfg(debug_assertions)]
static AGORA_API_TO_DATE: time::Date = date!(2012 - 01 - 07);
// #[cfg(not(debug_assertions))]
// static AGORA_API_TO_DATE: time::Date = time::Date::MAX;

#[derive(thiserror::Error, Debug)]
#[error("An error occurred while trying to get the agora api data")]
pub struct AgoraError(#[from] reqwest::Error);

pub async fn get_agora_api_data() -> Result<AgoraApiResponse, AgoraError> {
    let reqwest_client = reqwest::Client::new();

    let agora_response = reqwest_client
        .post(AGORA_API_URL)
        .header(AGORA_API_KEY_HEADER_NAME, AGORA_API_KEY_HEADER_VALUE)
        .body(
            serde_json::json! {{
                "filters": {
                    "from": AGORA_API_FROM_DATE,
                    "to": AGORA_API_TO_DATE,
                    "generation": [
                        GenerationKind::Biomass,
                        GenerationKind::GridEmissionFactor,
                        GenerationKind::HardCoal,
                        GenerationKind::Hydro,
                        GenerationKind::Lignite,
                        GenerationKind::NaturalGas,
                        GenerationKind::Nuclear,
                        GenerationKind::Other,
                        GenerationKind::PumpedStorageGeneration,
                        GenerationKind::Solar,
                        GenerationKind::TotalConventionalPowerPlant,
                        GenerationKind::TotalElectricityDemand,
                        GenerationKind::TotalGridEmissions,
                        GenerationKind::WindOffshore,
                        GenerationKind::WindOnshore,
                    ]
                },
                "x_coordinate": "date_id",
                "y_coordinate": "value",
                "view_name": "live_gen_plus_emi_de_hourly",
                "kpi_name": "power_generation",
                "z_coordinate": "generation"
            }}
            .to_string(),
        )
        .send()
        .await;

    Ok(agora_response?.json::<AgoraApiResponse>().await?)
}
