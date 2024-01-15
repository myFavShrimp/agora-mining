use std::sync::Arc;

use askama::Template;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use config::Config;
use database::power_generation_and_consumption::PowerGenerationAndConsumption;
use sqlx::PgPool;

mod agora;
mod config;
mod database;

#[derive(Template)]
#[template(path = "landingpage/landingpage.html")]
struct HelloAgoraTemplate<'a> {
    text: &'a str,
}

pub struct AppState {
    pub config: Config,
    pub postgres_pool: PgPool,
}

#[derive(Template)]
#[template(path = "about/about.html")]
struct AboutTemplate;

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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn landing_page_handler() -> impl IntoResponse {
    HelloAgoraTemplate {
        text: "by Denis, Hanna & Lucas",
    }
}

async fn refresh_data_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let agora_data = agora::get_agora_api_data().await;
    let agora_data = agora_data.unwrap().try_into().unwrap();

    PowerGenerationAndConsumption::create_many(&state.postgres_pool, agora_data).await;
    (StatusCode::OK, "Updated")
}

async fn about_page_handler() -> impl IntoResponse {
    AboutTemplate
}
