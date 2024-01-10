use serde::{Deserialize, Serialize};
use time::macros::date;

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
    data: Vec<(String, f64, String)>,
    columns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgoraApiResponse {
    status: bool,
    data: AgoraApiResponseData,
}

static AGORA_API_URL: &str = "https://api.agora-energy.org/api/raw-data";
static AGORA_API_KEY_HEADER_NAME: &str = "api-key";
static AGORA_API_KEY_HEADER_VALUE: &str = "agora_live_62ce76dd202927.67115829";

static AGORA_API_FROM_DATE: time::Date = date!(2012 - 01 - 01);

#[cfg(debug_assertions)]
static AGORA_API_TO_DATE: time::Date = date!(2012 - 01 - 07);
#[cfg(not(debug_assertions))]
static AGORA_API_TO_DATE: time::Date = time::Date::MAX;

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
