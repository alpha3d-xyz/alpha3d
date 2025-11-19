use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::{User, Order};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

pub async fn list_orders(
    Extension(user): Extension<User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    if user.role != "ADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let orders = sqlx::query_as::<_, Order>("SELECT * FROM orders ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(orders))
}

pub async fn update_order_status(
    Extension(user): Extension<User>,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<Order>, StatusCode> {
    if user.role != "ADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let order = sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = $1 WHERE id = $2 RETURNING *"
    )
    .bind(&payload.status)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(order))
}
