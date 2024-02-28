use sqlx::{migrate::MigrateDatabase, PgPool};
use time::PrimitiveDateTime;

pub mod power_emission;
pub mod power_generation;

pub trait Entity<F> {
    fn set_id(&mut self, date: PrimitiveDateTime);
    fn all_fields() -> Vec<F>;
    fn set_by_field(&mut self, field: F, value: f64);
    fn api_view_name() -> &'static str;
    fn api_kpi_name() -> &'static str;
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
