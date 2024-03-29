use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{
    macros::{date, format_description},
    PrimitiveDateTime,
};

use crate::database::{power_emission, power_generation, power_import_export, Entity};

#[derive(Serialize, Deserialize, Debug)]
pub struct AgoraApiResponseData<F> {
    pub data: Vec<(String, Option<f64>, F)>,
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
                existing_data.set_by_field(&kind, value);
            } else {
                let mut new_data = D::default();
                new_data.set_id(parsed_date);
                new_data.set_by_field(&kind, value);
                result_map.insert(parsed_date, new_data);
            }
        }

        Ok(result_map.into_values().collect())
    }
}

static AGORA_API_URL: &str = "https://api.agora-energy.org/api/raw-data";
static AGORA_API_KEY_HEADER_NAME: &str = "api-key";
static AGORA_API_KEY_HEADER_VALUE: &str = "agora_live_62ce76dd202927.67115829";

pub static AGORA_API_FROM_DATE: time::Date = date!(2012 - 01 - 01);

#[cfg(debug_assertions)]
static AGORA_API_TO_DATE: time::Date = date!(2012 - 01 - 07);
#[cfg(not(debug_assertions))]
static AGORA_API_TO_DATE: time::Date = time::Date::MAX;

#[derive(thiserror::Error, Debug)]
#[error("An error occurred while reading the agora api data")]
pub enum AgoraError {
    Http(#[from] reqwest::Error),
    TimeDeserialization(#[from] time::Error),
}

async fn get_agora_api_data<D, F>() -> Result<Vec<D>, AgoraError>
where
    D: Entity<F>,
    F: Serialize + for<'de> Deserialize<'de>,
    AgoraApiResponse<F>: TryInto<Vec<D>>,
    <AgoraApiResponse<F> as TryInto<Vec<D>>>::Error: std::fmt::Debug,
    AgoraError: From<<AgoraApiResponse<F> as TryInto<Vec<D>>>::Error>,
{
    let reqwest_client = reqwest::Client::new();

    let api_filter_values_key = D::api_filter_values_key();

    let agora_response = reqwest_client
        .post(AGORA_API_URL)
        .header(AGORA_API_KEY_HEADER_NAME, AGORA_API_KEY_HEADER_VALUE)
        .body(
            serde_json::json! {{
                "filters": {
                    "from": AGORA_API_FROM_DATE,
                    "to": AGORA_API_TO_DATE,
                    api_filter_values_key: D::all_fields()
                },
                "x_coordinate": "date_id",
                "y_coordinate": "value",
                "view_name": D::api_view_name(),
                "kpi_name": D::api_kpi_name(),
                "z_coordinate": api_filter_values_key
            }}
            .to_string(),
        )
        .send()
        .await;

    Ok(agora_response?
        .json::<AgoraApiResponse<F>>()
        .await?
        .try_into()?)
}

#[derive(thiserror::Error, Debug)]
#[error("An error occurred while syncing with the agora api")]
pub enum AgoraSyncError {
    Api(#[from] AgoraError),
    Database(#[from] sqlx::Error),
}

async fn sync_entity_with_agora_api<D, F>(
    connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), AgoraSyncError>
where
    D: Entity<F> + std::fmt::Debug,
    F: Serialize + for<'de> Deserialize<'de>,
    AgoraApiResponse<F>: TryInto<Vec<D>>,
    <AgoraApiResponse<F> as TryInto<Vec<D>>>::Error: std::fmt::Debug,
    AgoraError: From<<AgoraApiResponse<F> as TryInto<Vec<D>>>::Error>,
{
    let agora_data = get_agora_api_data::<D, F>().await?;

    D::delete_all(connection).await?;

    D::create_many(connection, agora_data).await?;

    Ok(())
}

pub async fn sync_all_entities_with_agora_api(connection: &PgPool) -> Result<(), AgoraSyncError> {
    async fn perform_sync(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), AgoraSyncError> {
        sync_entity_with_agora_api::<power_generation::PowerGeneration, power_generation::Fields>(
            connection,
        )
        .await?;
        sync_entity_with_agora_api::<power_emission::PowerEmission, power_emission::Fields>(
            connection,
        )
        .await?;
        sync_entity_with_agora_api::<
            power_import_export::PowerImportExport,
            power_import_export::Fields,
        >(connection)
        .await?;

        Ok(())
    }

    let mut transaction = connection.begin().await?;

    match perform_sync(&mut transaction).await {
        Ok(value) => {
            transaction.commit().await?;
            Ok(value)
        }
        Err(error) => {
            transaction.rollback().await?;
            Err(error)
        }
    }
}
