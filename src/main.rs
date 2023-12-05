use askama::Template;
use axum::{extract::Path, routing::get, Router};

#[derive(Template)]
#[template(path = "hello_agora.html")]
struct HelloAgoraTemplate<'a> {
    text: &'a str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        // credits doesnt get displayed
        .route("/", get(|| async { HelloAgoraTemplate { text: "by Denis, Hanna & Lucas" } }))
        .route("/hanna", get(|| async { HelloAgoraTemplate { text: "Das ist Hannas Seite - für Testzwecke" } }));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}



