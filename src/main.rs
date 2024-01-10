use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::macros::date;

#[derive(Template)]
#[template(path = "hello_agora.html")]
struct HelloAgoraTemplate<'a> {
    text: &'a str,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                HelloAgoraTemplate {
                    text: "by Denis, Hanna & Lucas",
                }
            }),
        )
        .route("/hanna", get(refresh_data_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
enum GenerationKind {
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
struct AgoraApiResponseData {
    data: Vec<(String, f64, String)>,
    columns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AgoraApiResponse {
    status: bool,
    data: AgoraApiResponseData,
}

static AGORA_API_URL: &str = "https://api.agora-energy.org/api/raw-data";
static AGORA_API_KEY_HEADER_NAME: &str = "api-key";
static AGORA_API_KEY_HEADER_VALUE: &str = "agora_live_62ce76dd202927.67115829";

static AGORA_API_FIRST_DATE: time::Date = date!(2012 - 01 - 01);

async fn refresh_data_handler() -> impl IntoResponse {
    let reqwest_client = reqwest::Client::new();

    let agora_response = reqwest_client
        .post(AGORA_API_URL)
        .header(AGORA_API_KEY_HEADER_NAME, AGORA_API_KEY_HEADER_VALUE)
        .body(
            json! {{
                "filters": {
                    "from": AGORA_API_FIRST_DATE,
                    "to": "2024-01-10",
                    "generation": [
                        "Biomass",
                        "Grid emission factor",
                        "Hard Coal",
                        "Hydro",
                        "Lignite",
                        "Natural Gas",
                        "Nuclear",
                        "Other",
                        "Pumped storage generation",
                        "Solar",
                        "Total conventional power plant",
                        "Total electricity demand",
                        "Total grid emissions",
                        "Wind offshore",
                        "Wind onshore"
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

    let agora_response_data = agora_response
        .unwrap()
        .json::<AgoraApiResponse>()
        .await
        .unwrap();

    (StatusCode::OK, Json(agora_response_data))
}
