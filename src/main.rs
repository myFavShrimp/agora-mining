use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Form, Router};
use config::Config;
use database::{agora_entities::AgoraEntities, power_generation, Entity};
use serde::Deserialize;
use sqlx::PgPool;
use templates::plotting::to_data_sets;
use time::{Date, Duration};

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
    data_sets: Vec<AgoraEntities>,
    #[serde(default = "default_from")]
    from: Date,
    #[serde(default = "default_to")]
    to: Date,
}

async fn graph_handler(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<GraphFormData>,
) -> impl IntoResponse {
    dbg!(&form_data);

    let result = power_generation::PowerGeneration::find_all_ordered_by_date(
        &state.postgres_pool,
        &form_data.from,
        &form_data.to,
    )
    .await;

    templates::PlottingTemplate {
        data_sets: to_data_sets(result.unwrap()),
        from: form_data.from,
        to: form_data.to,
    }
}

async fn refresh_data_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match dbg!(agora::sync_all_entities_with_agora_api(&state.postgres_pool).await) {
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
