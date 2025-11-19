use alpha3d::{create_app, AppState};
use alpha3d::models::{AuthResponse, User};
use alpha3d::storage::LocalStorage;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use sqlx::PgPool;
use serde_json::json;
use uuid::Uuid;
use http_body_util::BodyExt; // for collect
use std::sync::Arc;

// Helper to create a test pool
async fn get_test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    pool
}

#[tokio::test]
async fn test_signup_and_login() {
    let pool = get_test_pool().await;
    let storage = Arc::new(LocalStorage::new("./test_uploads"));
    let state = AppState { pool: pool.clone(), storage };
    let app = create_app(state);

    let email = format!("test_{}@example.com", Uuid::new_v4());
    let password = "password123";

    // 1. Signup
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/signup")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "email": email,
                    "password": password
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let user: User = serde_json::from_slice(&body).unwrap();
    assert_eq!(user.email, email);

    // 2. Login
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "email": email,
                    "password": password
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let auth_response: AuthResponse = serde_json::from_slice(&body).unwrap();
    let token = auth_response.token;

    // 3. Access Protected Route (Me)
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/auth/me")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let me: User = serde_json::from_slice(&body).unwrap();
    assert_eq!(me.email, email);
}
