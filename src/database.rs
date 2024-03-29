use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, PgPool};
use time::{Date, PrimitiveDateTime};

pub mod agora_entities;
pub mod power_emission;
pub mod power_generation;
pub mod power_import_export;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub enum Average {
    #[default]
    None,
    Daily,
    Monthly,
    Yearly,
}

impl Average {
    pub fn all() -> Vec<Self> {
        vec![
            Average::None,
            Average::Daily,
            Average::Monthly,
            Average::Yearly,
        ]
    }

    pub fn all_with_at_top(to_top: &Average) -> Vec<Self> {
        let all = Self::all();
        let mut result = Vec::with_capacity(all.len());
        result.push(to_top.clone());

        for item in all {
            if &item != to_top {
                result.push(item);
            }
        }

        result
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Average::None => "Nein",
            Average::Daily => "täglich",
            Average::Monthly => "monatlich",
            Average::Yearly => "jährlich",
        }
    }
}

pub trait Entity<F>: Sized {
    fn chart_display_name(field: &F) -> &'static str;
    fn unit(field: &F) -> String;
    fn set_id(&mut self, date: PrimitiveDateTime);
    fn id(&self) -> PrimitiveDateTime;
    fn all_fields() -> Vec<F>;
    fn set_by_field(&mut self, field: &F, value: Option<f64>);
    fn get_by_field(&self, field: &F) -> Option<f64>;
    fn api_view_name() -> &'static str;
    fn api_kpi_name() -> &'static str;
    fn api_filter_values_key() -> &'static str;

    async fn create(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        value: &Self,
    ) -> Result<Self, sqlx::Error>;
    async fn create_many(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        values: Vec<Self>,
    ) -> Result<Vec<Self>, sqlx::Error>;
    async fn delete_all(
        connection: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Vec<Self>, sqlx::Error>;
    async fn find_all_ordered_by_date(
        connection: &PgPool,
        from: &Date,
        to: &Date,
    ) -> Result<Vec<Self>, sqlx::Error>;
    async fn find_all_ordered_by_date_average_daily(
        connection: &PgPool,
        from: &Date,
        to: &Date,
    ) -> Result<Vec<Self>, sqlx::Error>;
    async fn find_all_ordered_by_date_average_monthly(
        connection: &PgPool,
        from: &Date,
        to: &Date,
    ) -> Result<Vec<Self>, sqlx::Error>;
    async fn find_all_ordered_by_date_average_yearly(
        connection: &PgPool,
        from: &Date,
        to: &Date,
    ) -> Result<Vec<Self>, sqlx::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseInitializationError {
    #[error("Could not connect to the database.")]
    Connection(sqlx::Error),
    #[error("Could not apply database migrations.")]
    Creation(sqlx::Error),
    #[error("Could not create the database.")]
    Migration(sqlx::migrate::MigrateError),
}

async fn create_database_if_not_exists(
    database_url: &str,
) -> Result<(), DatabaseInitializationError> {
    if !sqlx::Postgres::database_exists(database_url)
        .await
        .map_err(DatabaseInitializationError::Creation)?
    {
        sqlx::Postgres::create_database(database_url)
            .await
            .map_err(DatabaseInitializationError::Creation)?;
    };

    Ok(())
}

async fn apply_migrations(connection: &PgPool) -> Result<(), DatabaseInitializationError> {
    sqlx::migrate!()
        .run(connection)
        .await
        .map_err(DatabaseInitializationError::Migration)
}

pub async fn connect_and_migrate(
    database_url: &str,
) -> Result<PgPool, DatabaseInitializationError> {
    create_database_if_not_exists(database_url).await?;

    let connection =
        PgPool::connect_lazy(database_url).map_err(DatabaseInitializationError::Connection)?;
    apply_migrations(&connection).await?;

    Ok(connection)
}
