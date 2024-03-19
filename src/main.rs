use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_extra::extract::Query;
use config::Config;
use database::{agora_entities::AgoraEntities, Average};
use eyre::Context;
use serde::Deserialize;
use sqlx::PgPool;
use time::{Date, Duration};
use tower_http::trace::TraceLayer;
use tracing::Level;

mod agora;
mod config;
mod database;
mod templates;

pub struct AppState {
    pub config: Config,
    pub postgres_pool: PgPool,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    #[cfg(debug_assertions)]
    let trace_level = Level::TRACE;
    #[cfg(not(debug_assertions))]
    let trace_level = Level::INFO;

    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(trace_level)
            .finish(),
    )
    .wrap_err("Error initializing logging")?;

    let config = Config::from_env();
    let address = config.address();

    let state = Arc::new(AppState {
        postgres_pool: database::connect_and_migrate(&config.database_url).await?,
        config,
    });

    let app = Router::new()
        .route("/", get(landing_page_handler))
        .route("/about", get(about_page_handler))
        .route("/refresh", get(refresh_data_handler))
        .route("/graph", get(graph_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn landing_page_handler() -> impl IntoResponse {
    templates::LandingPageTemplate
}

fn default_data_sets() -> Vec<AgoraEntities> {
    vec![AgoraEntities::PowerGeneration]
}

fn default_to() -> Date {
    time::OffsetDateTime::now_utc().date()
}

fn default_from() -> Date {
    let from_date = time::OffsetDateTime::now_utc().date();

    from_date.checked_sub(Duration::days(30)).unwrap()
}

#[derive(Deserialize, Debug, Clone)]
struct GraphFormData {
    #[serde(default = "default_data_sets")]
    used_data_sets: Vec<AgoraEntities>,
    #[serde(default = "default_from")]
    from: Date,
    #[serde(default = "default_to")]
    to: Date,
    #[serde(default)]
    use_average: Average,
}

async fn graph_handler(
    State(state): State<Arc<AppState>>,
    Query(form_data): Query<GraphFormData>,
) -> impl IntoResponse {
    let result = AgoraEntities::plotting_data(
        &state.postgres_pool,
        &form_data.used_data_sets,
        &form_data.from,
        &form_data.to,
        &form_data.use_average,
    )
    .await;

    match result {
        Ok(plotting_data) => templates::PlottingTemplate {
            data_sets: plotting_data,
            from: form_data.from,
            to: form_data.to,
            used_data_sets: form_data.used_data_sets,
            use_average: form_data.use_average,
        }
        .into_response(),
        Err(e) => {
            tracing::error!(
                "\n{:?}",
                eyre::Report::new(e).wrap_err("Retrieving plotting data from database failed")
            );

            (StatusCode::OK, "Datenabfrage fehlgeschlagen").into_response()
        }
    }
}

async fn refresh_data_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match agora::sync_all_entities_with_agora_api(&state.postgres_pool).await {
        Ok(_) => (
            StatusCode::OK,
            [("HX-Retarget", format!("#{}", templates::REFRESH_BUTTON_ID))],
            "Updated",
        ),
        Err(e) => {
            tracing::error!(
                "\n{:?}",
                eyre::Report::new(e).wrap_err("Updating database from agora api failed")
            );

            (
                StatusCode::OK,
                [("HX-Retarget", format!("#{}", templates::REFRESH_BUTTON_ID))],
                "Update fehlgeschlagen",
            )
        }
    }
}

async fn about_page_handler() -> impl IntoResponse {
    templates::AboutTemplate
}
