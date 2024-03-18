use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use config::Config;
use database::{power_generation, Entity};
use sqlx::PgPool;
use templates::plotting::to_data_sets;
use time::Date;

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
        //.route("favicon")
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

//async fn fav_icon_handler() -> impl IntoResponse {}

async fn landing_page_handler() -> impl IntoResponse {
    templates::LandingPageTemplate
}

async fn graph_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result =
        power_generation::PowerGeneration::find_all_ordered_by_date(&state.postgres_pool).await;

    templates::PlottingTemplate {
        data_sets: to_data_sets(result.unwrap()),
        from: agora::AGORA_API_FROM_DATE,
        to: Date::MAX,
    }
}

async fn refresh_data_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match agora::sync_all_entities_with_agora_api(&state.postgres_pool).await {
        Ok(_) => (
            StatusCode::OK,
            [("HX-Retarget", format!("#{}", templates::REFRESH_BUTTON_ID))],
            "Updated",
        ),
        Err(_) => (
            StatusCode::OK,
            [("HX-Retarget", format!("#{}", templates::REFRESH_BUTTON_ID))],
            "Update Failed",
        ),
    }
}

async fn about_page_handler() -> impl IntoResponse {
    templates::AboutTemplate
}
