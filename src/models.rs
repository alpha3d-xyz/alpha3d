use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing, default)]
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: Uuid,
    pub file_id: Uuid,
    pub material: String,
    pub color: String,
    pub layer_height: f64,
    pub infill_percentage: i32,
    pub estimated_cost: f64, // DECIMAL in DB, f64 in Rust (sqlx handles this if feature enabled, or use BigDecimal)
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub quote_id: Uuid,
    pub status: String,
    pub shipping_address: sqlx::types::Json<serde_json::Value>,
    pub tracking_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub quote_id: Uuid,
    pub shipping_address: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderResponse {
    pub id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
