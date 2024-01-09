use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};

#[derive(Template)]
#[template(path = "hello_agora.html")]
struct HelloAgoraTemplate<'a> {
    text: &'a str,
}

#[tokio::main]
async fn main() {
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

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn refresh_data_handler() -> impl IntoResponse {
    println!("HANNA ist doof");
}
