use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::quoting::{calculate_quote, QuoteRequest};

#[derive(sqlx::FromRow)]
struct FileVolume {
    volume_cm3: Option<f64>,
}

pub async fn calculate_quote_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<QuoteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Fetch file volume
    let file = sqlx::query_as::<_, FileVolume>(
        "SELECT volume_cm3 FROM files WHERE id = $1"
    )
    .bind(payload.file_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let volume = match file {
        Some(f) => f.volume_cm3.ok_or((StatusCode::BAD_REQUEST, "File analysis not complete (volume missing)".to_string()))?,
        None => return Err((StatusCode::NOT_FOUND, "File not found".to_string())),
    };

    // 2. Calculate quote
    let mut response = calculate_quote(volume, &payload.material);

    // 3. Save quote to DB
    let quote_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO quotes (file_id, material, color, layer_height, infill_percentage, estimated_cost)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#
    )
    .bind(payload.file_id)
    .bind(format!("{:?}", payload.material).to_uppercase()) // Enum to string
    .bind(&payload.color)
    .bind(payload.layer_height.unwrap_or(0.2))
    .bind(payload.infill_percentage.unwrap_or(20))
    .bind(response.estimated_cost)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    response.id = quote_id;

    // 4. Return response
    Ok(Json(response))
}
