use async_trait::async_trait;
use bytes::Bytes;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[async_trait]
pub trait StorageService: Send + Sync {
    async fn upload_file(&self, file_name: &str, content: Bytes, content_type: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct LocalStorage {
    pub base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
        }
    }
}

#[async_trait]
impl StorageService for LocalStorage {
    async fn upload_file(&self, file_name: &str, content: Bytes, _content_type: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if !self.base_path.exists() {
            fs::create_dir_all(&self.base_path).await?;
        }
        let file_path = self.base_path.join(file_name);
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(&content).await?;
        Ok(format!("file://{}", file_path.to_string_lossy()))
    }
}

// GCS Storage - Simulated for testing without GCP credentials
// In production, integrate with google-cloud-storage v1.x+ API
pub struct GcsStorage {
    bucket_name: String,
    local_fallback: LocalStorage,
}

impl GcsStorage {
    pub async fn new(bucket_name: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Use local fallback for development/testing
        let fallback_path = format!("./uploads/gcs-simulation/{}", bucket_name);
        let local_fallback = LocalStorage::new(&fallback_path);
        
        Ok(Self { 
            bucket_name,
            local_fallback,
        })
    }
}

#[async_trait]
impl StorageService for GcsStorage {
    async fn upload_file(&self, file_name: &str, content: Bytes, content_type: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // For development: simulate GCS with local storage
        // In production: Replace with actual GCS API calls
        self.local_fallback.upload_file(file_name, content, content_type).await?;
        Ok(format!("gs://{}/{}", self.bucket_name, file_name))
    }
}
