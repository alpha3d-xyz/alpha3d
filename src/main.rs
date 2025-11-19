use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Serialize, ToSchema)]
struct GreetingResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct EchoPayload {
    message: String,
}

#[utoipa::path(
    get,
    path = "/api/greeting",
    tag = "Greeting",
    responses(
        (status = 200, description = "Returns a friendly greeting", body = GreetingResponse)
    )
)]
async fn get_greeting() -> Json<GreetingResponse> {
    Json(GreetingResponse {
        message: "Hello from Axum + Vue template".to_string(),
    })
}

#[utoipa::path(
    post,
    path = "/api/echo",
    tag = "Greeting",
    request_body = EchoPayload,
    responses((status = 200, description = "Echoes the message back", body = EchoPayload))
)]
async fn echo_message(Json(payload): Json<EchoPayload>) -> Json<EchoPayload> {
    Json(payload)
}

#[derive(OpenApi)]
#[openapi(
    paths(get_greeting, echo_message),
    components(schemas(GreetingResponse, EchoPayload)),
    tags((name = "Greeting", description = "Demo endpoints for the starter template"))
)]
struct ApiDoc;

fn build_api_router() -> Router {
    Router::new()
        .route("/api/greeting", get(get_greeting))
        .route("/api/echo", post(echo_message))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::permissive();

    let app = build_api_router()
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let address: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
