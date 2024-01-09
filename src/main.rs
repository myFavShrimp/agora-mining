use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "hello_agora.html")]
struct HelloAgoraTemplate<'a> {
    text: &'a str,
}

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

#[derive(Deserialize, Debug)]
struct AgoraApiResponseData {
    data: Vec<(String, f64, String)>,
    columns: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct AgoraApiResponse {
    status: bool,
    data: AgoraApiResponseData,
}

static AGORA_API_URL: &str = "https://api.agora-energy.org/api/raw-data";
static AGORA_API_KEY_HEADER_NAME: &str = "api-key";
static AGORA_API_KEY_HEADER_VALUE: &str = "agora_live_62ce76dd202927.67115829";

async fn refresh_data_handler() -> impl IntoResponse {
    println!("HANNA ist doof");

    let reqwest_client = reqwest::Client::new();

    let agora_response = reqwest_client
        .post(AGORA_API_URL)
        .header(AGORA_API_KEY_HEADER_NAME, AGORA_API_KEY_HEADER_VALUE)
        .body(
            r#"
            {
            	"filters": {
            		"from": "2014-01-01",
            		"to": "2015-01-01",
            		"generation": [
            			"Wind offshore",
            			"Wind onshore"
            		]
            	},
            	"x_coordinate": "date_id",
            	"y_coordinate": "value",
            	"view_name": "live_gen_plus_emi_de_hourly",
            	"kpi_name": "conventional_power_generation",
            	"z_coordinate": "generation"
            }
        "#,
        )
        .send()
        .await;

    println!(
        "{:#?}",
        agora_response.unwrap().json::<AgoraApiResponse>().await
    );

    (StatusCode::OK, "ok")
}
