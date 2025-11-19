use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::{User, Order};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AdminOrderListItem {
    pub id: Uuid,
    pub user_email: String,
    pub status: String,
    pub material: String,
    pub estimated_cost: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

pub async fn list_orders(
    Extension(user): Extension<User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<AdminOrderListItem>>, StatusCode> {
    if user.role != "ADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let orders = sqlx::query_as::<_, AdminOrderListItem>(
        r#"
        SELECT 
            o.id, 
            u.email as user_email, 
            o.status, 
            q.material, 
            q.estimated_cost::FLOAT8, 
            o.created_at
        FROM orders o
        JOIN users u ON o.user_id = u.id
        JOIN quotes q ON o.quote_id = q.id
        ORDER BY o.created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching orders: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

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
