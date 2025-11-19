# Google Cloud Storage Integration Guide

## Overview

The file upload system now supports pluggable storage backends through a `StorageService` trait. This allows seamless switching between local filesystem storage (for development) and Google Cloud Storage (for production).

## Implementation Details

### Storage Abstraction

```rust
#[async_trait]
pub trait StorageService: Send + Sync {
    async fn upload_file(
        &self, 
        file_name: &str, 
        content: Bytes, 
        content_type: &str
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
```

### Local Storage (`LocalStorage`)

- **Purpose**: Development and testing
- **Location**: `./uploads/` (configurable via `LOCAL_STORAGE_PATH`)
- **Returns**: `file:///absolute/path/to/file`

### GCS Storage (`GcsStorage`)

- **Current**: Simulated with local fallback
- **Location**: `./uploads/gcs-simulation/{bucket-name}/`
- **Returns**: `gs://{bucket-name}/{filename}`
- **Production Ready**: Structure in place for actual GCS API integration

## Usage

### Environment Variables

```bash
# Local Storage (default)
STORAGE_TYPE=local
LOCAL_STORAGE_PATH=./uploads

# GCS Storage (simulated)
STORAGE_TYPE=gcs
GCS_BUCKET_NAME=my-bucket
```

### Code Integration

The storage service is injected into the application state:

```rust
// src/main.rs
let storage: Arc<dyn StorageService> = if storage_type == "gcs" {
    Arc::new(GcsStorage::new(bucket_name).await?)
} else {
    Arc::new(LocalStorage::new(&path))
};

let app_state = AppState { pool, storage };
```

Handlers extract the storage service from state:

```rust
// src/handlers/files.rs
pub async fn upload_file(
    State(pool): State<PgPool>,
    State(storage): State<Arc<dyn StorageService>>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // ... validation ...
    
    let unique_filename = format!("{}_{}", Uuid::new_v4(), filename);
    let gcs_path = storage.upload_file(
        &unique_filename, 
        data.clone(), 
        &content_type
    ).await?;
    
    // ... save to database ...
}
```

## Testing

### Unit Tests

Storage backends can be mocked:

```rust
struct MockStorage;

#[async_trait]
impl StorageService for MockStorage {
    async fn upload_file(&self, file_name: &str, _: Bytes, _: &str) 
        -> Result<String, Box<dyn std::error::Error + Send + Sync>> 
    {
        Ok(format!("mock://{}", file_name))
    }
}
```

### Acceptance Tests

All acceptance tests work with both storage backends:

```bash
# Test with local storage
STORAGE_TYPE=local cargo test --test acceptance

# Test with GCS simulation
STORAGE_TYPE=gcs GCS_BUCKET_NAME=test-bucket cargo test --test acceptance
```

Test results show successful uploads with geometry analysis:

```text
test test_upload_all_stl_files_sorted ... ok
  Analyzed Cube_3d_printing_sample.stl: Volume=7.50 cm3, Surface Area=37.50 cm2
  Analyzed Menger_sponge_sample.stl: Volume=1480.56 cm3, Surface Area=3840.00 cm2
  Analyzed Stanford_Bunny_sample.stl: Volume=525.98 cm3, Surface Area=1204.64 cm2
  Analyzed Eiffel_tower_sample.STL: Volume=186652.34 cm3, Surface Area=247392.19 cm2
```

## Production GCS Integration

To enable real Google Cloud Storage:

### 1. Choose a GCS Client Library

The current implementation removed `google-cloud-storage` v1.4.0 due to API incompatibility. Consider:

- **gcloud-storage** (yoshidan's fork, proven API)
- **google-cloud-storage** v2.x+ (when available)
- Direct REST API with `reqwest`

### 2. Update Dependencies

```toml
[dependencies]
gcloud-storage = "0.x"  # Or your chosen library
```

### 3. Implement Real GCS Upload

```rust
// src/storage.rs
use gcloud_storage::client::{Client, ClientConfig};

impl GcsStorage {
    pub async fn new(bucket_name: String) -> Result<Self, Box<...>> {
        let config = ClientConfig::default().with_auth().await?;
        let client = Client::new(config);
        Ok(Self { client, bucket_name, local_fallback: None })
    }
}

#[async_trait]
impl StorageService for GcsStorage {
    async fn upload_file(...) -> Result<String, Box<...>> {
        let upload_type = UploadType::Simple(Media::new(file_name));
        let uploaded = self.client.upload_object(
            &UploadObjectRequest { bucket: self.bucket_name.clone(), .. },
            content.as_ref(),
            &upload_type,
        ).await?;
        Ok(format!("gs://{}/{}", uploaded.bucket, uploaded.name))
    }
}
```

### 4. Configure GCP Credentials

```bash
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json
export STORAGE_TYPE=gcs
export GCS_BUCKET_NAME=production-bucket
```

### 5. Test in Staging

```bash
cargo test --test acceptance
cargo run
```

## File Size Limits

- **Maximum**: 100 MB per file
- **Enforcement**: Handler-level check + Axum body limit
- **Error**: 413 Payload Too Large

```rust
const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100 MB

if data.len() > MAX_FILE_SIZE {
    return Err((StatusCode::PAYLOAD_TOO_LARGE, 
                "File size exceeds 100MB limit".to_string()));
}
```

## Database Integration

Files are tracked in the `files` table:

```sql
CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    filename TEXT NOT NULL,
    gcs_path TEXT NOT NULL,          -- Storage path (file:// or gs://)
    file_size_bytes BIGINT NOT NULL,
    volume_cm3 NUMERIC(10,2),
    surface_area_cm2 NUMERIC(10,2),
    status TEXT NOT NULL DEFAULT 'UPLOADED',
    created_at TIMESTAMP DEFAULT NOW()
);
```

## Benefits

✅ **Testable**: Local storage for tests, no GCP credentials needed  
✅ **Flexible**: Easy to switch backends via environment variables  
✅ **Production-Ready**: Clear path to GCS integration  
✅ **Type-Safe**: Trait-based abstraction with async support  
✅ **Monitored**: All uploads logged with tracing  

## Next Steps

1. ✅ Storage abstraction implemented
2. ✅ Local storage working
3. ✅ GCS simulation working
4. ✅ Acceptance tests passing
5. ⏳ Production GCS client integration (when needed)
6. ⏳ Add download endpoints
7. ⏳ Implement file deletion
