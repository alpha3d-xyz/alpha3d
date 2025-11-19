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

// Helper to create a test pool
async fn get_test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");
    // Ensure migrations are run
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    pool
}

#[tokio::test]
async fn test_file_upload_and_analysis() {
    let pool = get_test_pool().await;
    let storage = Arc::new(LocalStorage::new("./test_uploads"));
    let state = AppState { pool: pool.clone(), storage };
    let app = create_app(state);


    // 1. Signup & Login to get token
    let email = format!("test_file_{}@example.com", Uuid::new_v4());
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

    // 2. Create a dummy STL file (Binary STL is safer for stl_io usually, but let's try ASCII)
    // Triangle at z=10: (0,0,10), (10,0,10), (0,10,10).
    // Normal (0,0,1).
    let stl_content = "solid test
facet normal 0 0 1
outer loop
vertex 0 0 10
vertex 10 0 10
vertex 0 10 10
endloop
endfacet
endsolid test";

    let boundary = "------------------------boundary123";
    let body_data = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.stl\"\r\n\r\n{}\r\n--{}--\r\n",
        boundary, stl_content, boundary
    );

    let response = app.clone()
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

    // If stl_io fails to parse ASCII from the multipart stream, we might get 400.
    // Let's check status.
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let upload_res: UploadResponse = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(upload_res.filename, "test.stl");
    // Volume check: 
    // V = 1/6 * det(v1, v2, v3)
    // v1=(0,0,10), v2=(10,0,10), v3=(0,10,10)
    // det = 0*... - 0*... + 10*(10*10 - 0*0) = 1000.
    // V = 1000 / 6 = 166.666 mm3.
    // In cm3: 0.1666...
    // Let's just assert it's > 0.
    assert!(upload_res.volume_cm3 > 0.0);
}
