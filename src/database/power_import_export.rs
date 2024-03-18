use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{Date, PrimitiveDateTime};

use super::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerImportExport {
    pub date_id: PrimitiveDateTime,
    pub poland: Option<f64>,
    pub france: Option<f64>,
    pub norway: Option<f64>,
    pub denmark: Option<f64>,
    pub sweden: Option<f64>,
    pub austria: Option<f64>,
    pub belgium: Option<f64>,
    pub netherlands: Option<f64>,
    pub czech: Option<f64>,
    pub luxembourg: Option<f64>,
    pub switzerland: Option<f64>,
    pub net_total: Option<f64>,
    pub power_price: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Fields {
    #[serde(rename = "PL")]
    Poland,
    #[serde(rename = "FR")]
    France,
    #[serde(rename = "NO")]
    Norway,
    #[serde(rename = "DK")]
    Denmark,
    #[serde(rename = "SE")]
    Sweden,
    #[serde(rename = "AT")]
    Austria,
    #[serde(rename = "BE")]
    Belgium,
    #[serde(rename = "NL")]
    Netherlands,
    #[serde(rename = "CZ")]
    Czech,
    #[serde(rename = "LU")]
    Luxembourg,
    #[serde(rename = "CH")]
    Switzerland,
    #[serde(rename = "Net Total")]
    NetTotal,
    #[serde(rename = "Power price")]
    PowerPrice,
}

impl Entity<Fields> for PowerImportExport {
    fn unit(field: &Fields) -> String {
        match field {
            Fields::Poland
            | Fields::France
            | Fields::Norway
            | Fields::Denmark
            | Fields::Sweden
            | Fields::Austria
            | Fields::Belgium
            | Fields::Netherlands
            | Fields::Czech
            | Fields::Luxembourg
            | Fields::Switzerland
            | Fields::NetTotal => "Gwh/hour".to_owned(),
            Fields::PowerPrice => "EUR/MWh".to_owned(),
        }
    }

    fn all_fields() -> Vec<Fields> {
        vec![
            Fields::Poland,
            Fields::France,
            Fields::Norway,
            Fields::Denmark,
            Fields::Sweden,
            Fields::Austria,
            Fields::Belgium,
            Fields::Netherlands,
            Fields::Czech,
            Fields::Luxembourg,
            Fields::Switzerland,
            Fields::NetTotal,
            Fields::PowerPrice,
        ]
    }

    fn set_by_field(&mut self, field: &Fields, value: Option<f64>) {
        match field {
            Fields::Poland => self.poland = value,
            Fields::France => self.france = value,
            Fields::Norway => self.norway = value,
            Fields::Denmark => self.denmark = value,
            Fields::Sweden => self.sweden = value,
            Fields::Austria => self.austria = value,
            Fields::Belgium => self.belgium = value,
            Fields::Netherlands => self.netherlands = value,
            Fields::Czech => self.czech = value,
            Fields::Luxembourg => self.luxembourg = value,
            Fields::Switzerland => self.switzerland = value,
            Fields::NetTotal => self.net_total = value,
            Fields::PowerPrice => self.power_price = value,
        }
    }

    fn get_by_field(&self, field: &Fields) -> Option<f64> {
        match field {
            Fields::Poland => self.poland,
            Fields::France => self.france,
            Fields::Norway => self.norway,
            Fields::Denmark => self.denmark,
            Fields::Sweden => self.sweden,
            Fields::Austria => self.austria,
            Fields::Belgium => self.belgium,
            Fields::Netherlands => self.netherlands,
            Fields::Czech => self.czech,
            Fields::Luxembourg => self.luxembourg,
            Fields::Switzerland => self.switzerland,
            Fields::NetTotal => self.net_total,
            Fields::PowerPrice => self.power_price,
        }
    }

    fn set_id(&mut self, date: PrimitiveDateTime) {
        self.date_id = date
    }

    fn api_view_name() -> &'static str {
        "live_exchange_plus_price_de_hourly"
    }

    fn api_kpi_name() -> &'static str {
        "power_import_export"
    }

    fn api_filter_values_key() -> &'static str {
        "legends"
    }

    async fn create(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        value: &PowerImportExport,
    ) -> Result<PowerImportExport, sqlx::Error> {
        sqlx::query_as!(
            PowerImportExport,
            "
                INSERT INTO power_import_export 
                    (date_id, poland, france, norway, denmark, sweden, austria, belgium, netherlands, czech, luxembourg, switzerland, net_total, power_price)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                RETURNING *
            ",
            value.date_id,
            value.poland,
            value.france,
            value.norway,
            value.denmark,
            value.sweden,
            value.austria,
            value.belgium,
            value.netherlands,
            value.czech,
            value.luxembourg,
            value.switzerland,
            value.net_total,
            value.power_price,
        ).fetch_one(&mut **connection).await
    }

    async fn create_many(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        values: Vec<PowerImportExport>,
    ) -> Result<Vec<PowerImportExport>, sqlx::Error> {
        let mut result = Vec::new();

        for item in values {
            result.push(PowerImportExport::create(connection, &item).await?);
        }

        Ok(result)
    }

    async fn delete_all(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Vec<PowerImportExport>, sqlx::Error> {
        sqlx::query_as!(
            PowerImportExport,
            "
                DELETE FROM power_import_export 
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
    ) -> Result<Vec<PowerImportExport>, sqlx::Error> {
        sqlx::query_as!(
            PowerImportExport,
            "
                SELECT * FROM power_import_export WHERE date_id >= $1 AND date_id <= $2 ORDER BY date_id ASC
            ",
            from.midnight(),
            to.midnight(), // TODO: last minute of day
        )
        .fetch_all(connection)
        .await
    }
}

impl Default for PowerImportExport {
    fn default() -> Self {
        Self {
            date_id: PrimitiveDateTime::MIN,
            poland: Default::default(),
            france: Default::default(),
            norway: Default::default(),
            denmark: Default::default(),
            sweden: Default::default(),
            austria: Default::default(),
            belgium: Default::default(),
            netherlands: Default::default(),
            czech: Default::default(),
            luxembourg: Default::default(),
            switzerland: Default::default(),
            net_total: Default::default(),
            power_price: Default::default(),
        }
    }
}
