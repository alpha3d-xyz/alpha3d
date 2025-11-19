use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use alpha3d::{create_app, AppState, storage::{LocalStorage, GcsStorage, StorageService}};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let storage_type = env::var("STORAGE_TYPE").unwrap_or_else(|_| "local".to_string());
    let storage: Arc<dyn StorageService> = if storage_type == "gcs" {
        let bucket_name = env::var("GCS_BUCKET_NAME").unwrap_or_else(|_| "my-bucket".to_string());
        tracing::info!("Using GCS storage (simulated) with bucket: {}", bucket_name);
        Arc::new(GcsStorage::new(bucket_name).await.expect("Failed to create GCS storage"))
    } else {
        let path = env::var("LOCAL_STORAGE_PATH").unwrap_or_else(|_| "./uploads".to_string());
        tracing::info!("Using local storage at: {}", path);
        Arc::new(LocalStorage::new(&path))
    };

    let app_state = AppState {
        pool,
        storage,
    };

    let app = create_app(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
