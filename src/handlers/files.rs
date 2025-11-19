use axum::{
    extract::{State, Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    Json, Extension,
};
use sqlx::PgPool;
use crate::models::User;
use crate::analysis;
use crate::storage::StorageService;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100 MB

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
    State(storage): State<Arc<dyn StorageService>>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let filename = field.file_name().unwrap_or("unknown.stl").to_string();
        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let data = field.bytes().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if data.len() > MAX_FILE_SIZE {
            return Err((StatusCode::PAYLOAD_TOO_LARGE, "File size exceeds 100MB limit".to_string()));
        }

        // 1. Analyze Geometry
        let analysis = analysis::analyze_stl(&data).map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid STL: {}", e)))?;

        // 2. Upload to Storage
        let unique_filename = format!("{}_{}", Uuid::new_v4(), filename);
        let gcs_path = storage.upload_file(&unique_filename, data.clone(), &content_type).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Storage error: {}", e)))?;

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
    Extension(_user): Extension<User>,
    Path(file_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let file = sqlx::query_as::<_, FileRecord>(
        "SELECT id, filename, volume_cm3, surface_area_cm2, status, created_at FROM files WHERE id = $1"
    )
    .bind(file_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match file {
        Some(f) => Ok(Json(f)),
        None => Err((StatusCode::NOT_FOUND, "File not found".to_string())),
    }
}

pub async fn get_file_quoting(
    State(_pool): State<PgPool>,
    Extension(_user): Extension<User>,
    Path(_file_id): Path<Uuid>,
) -> Result<Json<()>, (StatusCode, String)> {
    // Placeholder for now
    Err((StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()))
}
