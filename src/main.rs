use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use config::Config;

mod agora;
mod config;
mod database;

#[derive(Template)]
#[template(path = "hello_agora.html")]
struct HelloAgoraTemplate<'a> {
    text: &'a str,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = Config::from_env();
    let address = config.address();

    let _postgres_pool = database::connect_and_migrate(&config.database_url).await?;

    let app = Router::new()
        .route("/", get(landing_page_handler))
        .route("/hanna", get(refresh_data_handler));

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn landing_page_handler() -> impl IntoResponse {
    HelloAgoraTemplate {
        text: "by Denis, Hanna & Lucas",
    }
}

async fn refresh_data_handler() -> impl IntoResponse {
    let _agora_data = agora::get_agora_api_data().await;

    (StatusCode::OK, "Updated")
}
