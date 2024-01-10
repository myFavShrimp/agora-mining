use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[derive(Template)]
#[template(path = "hello_agora.html")]
struct HelloAgoraTemplate<'a> {
    text: &'a str,
}

mod agora;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                HelloAgoraTemplate {
                    text: "by Denis, Hanna & Lucas",
                }
            }),
        )
        .route("/hanna", get(refresh_data_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn refresh_data_handler() -> impl IntoResponse {
    let _agora_data = agora::get_agora_api_data().await;

    (StatusCode::OK, "Updated")
}
