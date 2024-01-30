use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use config::Config;
use database::power_generation_and_consumption::PowerGenerationAndConsumption;
use sqlx::PgPool;

mod agora;
mod config;
mod database;
mod templates;
mod plotting;

pub struct AppState {
    pub config: Config,
    pub postgres_pool: PgPool,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = Config::from_env();
    let address = config.address();

    let state = Arc::new(AppState {
        postgres_pool: database::connect_and_migrate(&config.database_url).await?,
        config,
    });

    let app = Router::new()
        .route("/", get(landing_page_handler))
        .route("/about", get(about_page_handler))
        .route("/hanna", get(refresh_data_handler))
        .route("/graph-dracula", get(graph_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn landing_page_handler() -> impl IntoResponse {
    templates::HelloAgoraTemplate {
        text: "by Denis, Hanna & Lucas",
    }
}

async fn graph_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = PowerGenerationAndConsumption::find_all(&state.postgres_pool).await;
    plotting::create_plot(result.unwrap())
}

async fn refresh_data_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let agora_data = agora::get_agora_api_data().await;
    let agora_data = agora_data.unwrap().try_into().unwrap();

    _ = PowerGenerationAndConsumption::delete_all(&state.postgres_pool).await;

    match PowerGenerationAndConsumption::create_many(&state.postgres_pool, agora_data).await {
        Ok(_) => (
            StatusCode::OK,
            [("HX-Retarget", format!("#{}", templates::REFRESH_BUTTON_ID))],
            "Updated",
        ),
        Err(_) => (
            StatusCode::OK,
            [("HX-Retarget", format!("#{}", templates::REFRESH_BUTTON_ID))],
            "Failed!",
        ),
    }
}

async fn about_page_handler() -> impl IntoResponse {
    templates::AboutTemplate
}
