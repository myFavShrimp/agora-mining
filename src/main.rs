use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use config::Config;
use database::power_generation_and_consumption::PowerGenerationAndConsumption;
use sqlx::PgPool;

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

async fn graph_handler() -> impl IntoResponse {
    use plotters::prelude::*;
    let mut svg_path = String::new();

    {
        let root = SVGBackend::with_string(&mut svg_path, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32).unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            )).unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();

        root.present().unwrap();

    };
    svg_path
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
