pub mod auth;
pub mod handlers;
pub mod models;
pub mod middleware;
pub mod analysis;
pub mod storage;
pub mod quoting;

use axum::{
    Json, Router, Extension,
    routing::{get, post},
    middleware::from_fn,
    extract::{DefaultBodyLimit, FromRef},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use sqlx::PgPool;
use std::sync::Arc;
use storage::StorageService;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub storage: Arc<dyn StorageService>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Arc<dyn StorageService> {
    fn from_ref(state: &AppState) -> Self {
        state.storage.clone()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GreetingResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EchoPayload {
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/api/greeting",
    tag = "Greeting",
    responses(
        (status = 200, description = "Returns a friendly greeting", body = GreetingResponse)
    )
)]
pub async fn get_greeting() -> Json<GreetingResponse> {
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
pub async fn echo_message(Json(payload): Json<EchoPayload>) -> Json<EchoPayload> {
    Json(payload)
}

#[derive(OpenApi)]
#[openapi(
    paths(get_greeting, echo_message),
    components(schemas(GreetingResponse, EchoPayload)),
    tags(
        (name = "Greeting", description = "Greeting API")
    )
)]
pub struct ApiDoc;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api/greeting", get(get_greeting))
        .route("/api/echo", post(echo_message))
        .route("/api/auth/signup", post(handlers::signup))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/me", get(handlers::me).layer(from_fn(middleware::auth_middleware)))
        .route("/api/files/upload", post(handlers::files::upload_file).layer(from_fn(middleware::auth_middleware)))
        .route("/api/files/:id/analysis", get(handlers::files::get_file_analysis).layer(from_fn(middleware::auth_middleware)))
        .route("/api/files/:id/quoting", get(handlers::files::get_file_quoting).layer(from_fn(middleware::auth_middleware)))
        .route("/api/quotes/calculate", post(handlers::quoting::calculate_quote_handler).layer(from_fn(middleware::auth_middleware)))
        .layer(DefaultBodyLimit::max(102 * 1024 * 1024))
        .layer(Extension(state.pool.clone()))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
