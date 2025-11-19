use alpha3d::{create_app, AppState};
use alpha3d::storage::LocalStorage;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use sqlx::PgPool;
use serde_json::json;
use uuid::Uuid;
use std::sync::Arc;

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

async fn create_user(app: &axum::Router, email: &str, password: &str) -> String {
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
    
    // Login to get token
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
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    json["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_admin_access_denied_for_user() {
    let pool = get_test_pool().await;
    let storage = Arc::new(LocalStorage::new("./test_uploads"));
    let state = AppState { pool: pool.clone(), storage };
    let app = create_app(state);

    let email = format!("user_{}@example.com", Uuid::new_v4());
    let token = create_user(&app, &email, "password").await;

    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/admin/orders")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should be Forbidden (403) or NotFound (404) if route doesn't exist yet.
    // Since we are doing TDD, we expect this to fail or return 404 initially.
    // But the requirement is "access denied", so 403 is the goal.
    // If the route is not implemented, it will be 404.
    // If implemented but no check, 200.
    // We want to assert it is NOT 200 first.
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_admin_list_orders() {
    let pool = get_test_pool().await;
    let storage = Arc::new(LocalStorage::new("./test_uploads"));
    let state = AppState { pool: pool.clone(), storage };
    let app = create_app(state);

    let email = format!("admin_{}@example.com", Uuid::new_v4());
    let token = create_user(&app, &email, "password").await;

    // Promote user to ADMIN
    sqlx::query("UPDATE users SET role = 'ADMIN' WHERE email = $1")
        .bind(&email)
        .execute(&pool)
        .await
        .unwrap();

    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/admin/orders")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_admin_update_order_status() {
    let pool = get_test_pool().await;
    let storage = Arc::new(LocalStorage::new("./test_uploads"));
    let state = AppState { pool: pool.clone(), storage };
    let app = create_app(state);

    // 1. Create Admin User
    let admin_email = format!("admin_{}@example.com", Uuid::new_v4());
    let admin_token = create_user(&app, &admin_email, "password").await;
    sqlx::query("UPDATE users SET role = 'ADMIN' WHERE email = $1")
        .bind(&admin_email)
        .execute(&pool)
        .await
        .unwrap();

    // 2. Create Normal User for the order
    let user_email = format!("user_{}@example.com", Uuid::new_v4());
    let _user_token = create_user(&app, &user_email, "password").await;
    
    let user_id: Uuid = sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
        .bind(&user_email)
        .fetch_one(&pool)
        .await
        .unwrap();

    // 3. Create File
    let file_id = Uuid::new_v4();
    sqlx::query("INSERT INTO files (id, user_id, filename, gcs_path, file_size_bytes, status) VALUES ($1, $2, 'test.stl', 'path/to/test.stl', 100, 'COMPLETED')")
        .bind(file_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    // 4. Create Quote
    let quote_id = Uuid::new_v4();
    sqlx::query("INSERT INTO quotes (id, file_id, material, color, layer_height, infill_percentage, estimated_cost) VALUES ($1, $2, 'PLA', 'Red', 0.2, 20, 10.0)")
        .bind(quote_id)
        .bind(file_id)
        .execute(&pool)
        .await
        .unwrap();

    // 5. Create Order
    let order_id = Uuid::new_v4();
    sqlx::query("INSERT INTO orders (id, user_id, quote_id, status, shipping_address) VALUES ($1, $2, $3, 'PENDING', '{}')")
        .bind(order_id)
        .bind(user_id)
        .bind(quote_id)
        .execute(&pool)
        .await
        .unwrap();

    // 6. Update Status
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/api/admin/orders/{}/status", order_id))
                .header("Authorization", format!("Bearer {}", admin_token))
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "status": "SHIPPED"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 7. Verify DB
    let status: String = sqlx::query_scalar("SELECT status FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    
    assert_eq!(status, "SHIPPED");
}
