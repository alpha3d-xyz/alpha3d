use axum::{
    extract::{State, Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    Json, Extension,
};
use sqlx::PgPool;
use crate::models::User;
use crate::analysis;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct FileRecord {
    pub id: Uuid,
    pub filename: String,
    pub volume_cm3: Option<f64>,
    pub surface_area_cm2: Option<f64>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub file_id: Uuid,
    pub filename: String,
    pub volume_cm3: f64,
    pub surface_area_cm2: f64,
}

pub async fn upload_file(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let filename = field.file_name().unwrap_or("unknown.stl").to_string();
        let data = field.bytes().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // 1. Analyze Geometry
        let analysis = analysis::analyze_stl(&data).map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid STL: {}", e)))?;

        // 2. Save to DB (Simulating GCS upload by just storing metadata for now)
        // In a real app, we would upload `data` to GCS here and get the `gcs_path`.
        let gcs_path = format!("gs://bucket/{}", filename); 

        let file_record = sqlx::query_as::<_, FileRecord>(
            r#"
            INSERT INTO files (user_id, filename, gcs_path, file_size_bytes, volume_cm3, surface_area_cm2, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'ANALYZED')
            RETURNING id, filename, volume_cm3, surface_area_cm2, status, created_at
            "#
        )
        .bind(user.id)
        .bind(&filename)
        .bind(gcs_path)
        .bind(data.len() as i64)
        .bind(analysis.volume_cm3)
        .bind(analysis.surface_area_cm2)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok((StatusCode::CREATED, Json(UploadResponse {
            file_id: file_record.id,
            filename: file_record.filename,
            volume_cm3: file_record.volume_cm3.unwrap_or(0.0),
            surface_area_cm2: file_record.surface_area_cm2.unwrap_or(0.0),
        })));
    }

    Err((StatusCode::BAD_REQUEST, "No file provided".to_string()))
}

pub async fn get_file_analysis(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Path(file_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let file = sqlx::query_as::<_, FileRecord>(
        "SELECT id, filename, volume_cm3, surface_area_cm2, status, created_at FROM files WHERE id = $1 AND user_id = $2"
    )
    .bind(file_id)
    .bind(user.id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "File not found".to_string()))?;

    Ok(Json(file))
}
