use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use sqlx::PgPool;
use crate::models::{User, CreateOrderRequest, OrderResponse};

pub async fn create_order(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Verify quote exists
    let quote_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM quotes WHERE id = $1)"
    )
    .bind(payload.quote_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !quote_exists {
        return Err((StatusCode::NOT_FOUND, "Quote not found".to_string()));
    }

    // 2. Create Order
    let order = sqlx::query_as::<_, OrderResponse>(
        r#"
        INSERT INTO orders (user_id, quote_id, status, shipping_address)
        VALUES ($1, $2, 'PAID', $3)
        RETURNING id, status, created_at
        "#
    )
    .bind(user.id)
    .bind(payload.quote_id)
    .bind(sqlx::types::Json(payload.shipping_address))
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn list_orders(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let orders = sqlx::query_as::<_, OrderResponse>(
        "SELECT id, status, created_at FROM orders WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(orders))
}
