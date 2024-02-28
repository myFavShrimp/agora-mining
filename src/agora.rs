use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{
    macros::{date, format_description},
    PrimitiveDateTime,
};

use crate::database::Entity;

#[derive(Serialize, Deserialize, Debug)]
pub struct AgoraApiResponseData<F> {
    pub data: Vec<(String, f64, F)>,
    pub columns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgoraApiResponse<F> {
    pub status: bool,
    pub data: AgoraApiResponseData<F>,
}

impl<D, F> TryInto<Vec<D>> for AgoraApiResponse<F>
where
    D: Entity<F> + Default,
    F: Serialize,
{
    type Error = time::Error;

    fn try_into(self) -> Result<Vec<D>, Self::Error> {
        let data = self.data.data;

        let mut result_map: HashMap<PrimitiveDateTime, D> = HashMap::new();

        let format_description =
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

        for item in data {
            let (date, value, kind) = item;
            let parsed_date = PrimitiveDateTime::parse(&date, format_description)?;

            if let Some(existing_data) = result_map.get_mut(&parsed_date) {
                existing_data.set_by_field(kind, value);
            } else {
                let mut new_data = D::default();
                new_data.set_id(parsed_date);
                new_data.set_by_field(kind, value);
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

async fn get_agora_api_data<D, F>() -> Result<AgoraApiResponse<F>, AgoraError>
where
    D: Entity<F>,
    F: Serialize + for<'de> Deserialize<'de>,
{
    let reqwest_client = reqwest::Client::new();

    let agora_response = reqwest_client
        .post(AGORA_API_URL)
        .header(AGORA_API_KEY_HEADER_NAME, AGORA_API_KEY_HEADER_VALUE)
        .body(
            serde_json::json! {{
                "filters": {
                    "from": AGORA_API_FROM_DATE,
                    "to": AGORA_API_TO_DATE,
                    "generation": D::all_fields()
                },
                "x_coordinate": "date_id",
                "y_coordinate": "value",
                "view_name": D::api_view_name(),
                "kpi_name": D::api_kpi_name(),
                "z_coordinate": "generation"
            }}
            .to_string(),
        )
        .send()
        .await;

    Ok(agora_response?.json::<AgoraApiResponse<F>>().await?)
}

pub async fn sync_entity_with_agora_api<D, F>(connection: &PgPool) -> Result<(), AgoraError>
where
    D: Entity<F>,
    F: Serialize + for<'de> Deserialize<'de>,
    AgoraApiResponse<F>: TryInto<Vec<D>>,
    <AgoraApiResponse<F> as TryInto<Vec<D>>>::Error: std::fmt::Debug,
{
    let agora_data = get_agora_api_data::<D, F>().await;
    let agora_data: Vec<D> = agora_data.unwrap().try_into().unwrap();

    _ = D::delete_all(connection).await;

    match D::create_many(&connection, agora_data).await {
        Ok(_) => {}
        Err(_) => {}
    }

    Ok(())
}
