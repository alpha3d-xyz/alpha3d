use alpha3d::{create_app, AppState};
use alpha3d::models::AuthResponse;
use alpha3d::handlers::files::UploadResponse;
use alpha3d::storage::LocalStorage;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use sqlx::PgPool;
use serde_json::json;
use uuid::Uuid;
use http_body_util::BodyExt;
use std::sync::Arc;
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    id: Uuid,
    estimated_cost: f64,
    currency: String,
    breakdown: CostBreakdown,
}

#[derive(Debug, Deserialize)]
struct CostBreakdown {
    material_cost: f64,
    machine_cost: f64,
    labor_cost: f64,
}

#[tokio::test]
async fn test_create_quote() {
    let pool = get_test_pool().await;
    let storage = Arc::new(LocalStorage::new("./test_uploads"));
    let state = AppState { pool: pool.clone(), storage };
    let app = create_app(state);

    // 1. Signup & Login to get token
    let email = format!("test_quote_{}@example.com", Uuid::new_v4());
    let password = "password123";

    let _ = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/signup")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "email": email, "password": password }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let login_res = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "email": email, "password": password }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let body = login_res.into_body().collect().await.unwrap().to_bytes();
    let auth_response: AuthResponse = serde_json::from_slice(&body).unwrap();
    let token = auth_response.token;

    // 2. Upload a file to get file_id
    // Simple Cube 10x10x10
    let stl_content = "solid cube
facet normal 0 0 -1
outer loop
vertex 0 0 0
vertex 10 0 0
vertex 0 10 0
endloop
endfacet
facet normal 0 0 -1
outer loop
vertex 0 10 0
vertex 10 0 0
vertex 10 10 0
endloop
endfacet
facet normal 0 0 1
outer loop
vertex 0 0 10
vertex 10 0 10
vertex 0 10 10
endloop
endfacet
facet normal 0 0 1
outer loop
vertex 0 10 10
vertex 10 0 10
vertex 10 10 10
endloop
endfacet
facet normal 0 -1 0
outer loop
vertex 0 0 0
vertex 10 0 0
vertex 0 0 10
endloop
endfacet
facet normal 0 -1 0
outer loop
vertex 0 0 10
vertex 10 0 0
vertex 10 0 10
endloop
endfacet
facet normal 0 1 0
outer loop
vertex 0 10 0
vertex 10 10 0
vertex 0 10 10
endloop
endfacet
facet normal 0 1 0
outer loop
vertex 0 10 10
vertex 10 10 0
vertex 10 10 10
endloop
endfacet
facet normal -1 0 0
outer loop
vertex 0 0 0
vertex 0 10 0
vertex 0 0 10
endloop
endfacet
facet normal -1 0 0
outer loop
vertex 0 0 10
vertex 0 10 0
vertex 0 10 10
endloop
endfacet
facet normal 1 0 0
outer loop
vertex 10 0 0
vertex 10 10 0
vertex 10 0 10
endloop
endfacet
facet normal 1 0 0
outer loop
vertex 10 0 10
vertex 10 10 0
vertex 10 10 10
endloop
endfacet
endsolid cube";

    let boundary = "------------------------boundary123";
    let body_data = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"cube.stl\"\r\n\r\n{}\r\n--{}--\r\n",
        boundary, stl_content, boundary
    );

    let upload_res = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/files/upload")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", format!("multipart/form-data; boundary={}", boundary))
                .body(Body::from(body_data))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(upload_res.status(), StatusCode::CREATED);
    let upload_body = upload_res.into_body().collect().await.unwrap().to_bytes();
    let upload_response: UploadResponse = serde_json::from_slice(&upload_body).unwrap();
    let file_id = upload_response.file_id;

    // 3. Calculate Quote
    let quote_payload = json!({
        "file_id": file_id,
        "material": "PLA",
        "color": "Red"
    });

    let quote_res = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/quotes/calculate")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(quote_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(quote_res.status(), StatusCode::OK);

    let quote_body = quote_res.into_body().collect().await.unwrap().to_bytes();
    let quote_response: QuoteResponse = serde_json::from_slice(&quote_body).unwrap();

    assert_eq!(quote_response.currency, "KRW");
    assert!(quote_response.estimated_cost > 0.0);
    
    // Volume of 10x10x10 cube is 1000 mm3 = 1 cm3.
    // Wait, 10 units. If units are mm, then 10mm x 10mm x 10mm = 1000 mm3 = 1 cm3.
    // If units are cm, then 1000 cm3.
    // STL usually doesn't have units, but we assume mm usually.
    // My analysis code likely assumes units are consistent.
    // Let's check what volume I get.
    println!("Volume: {}", upload_response.volume_cm3);
}
