# Storage Configuration

## Environment Variables

### `STORAGE_TYPE`

Controls which storage backend is used for uploaded files.

- **Default**: `local`
- **Options**: `local` | `gcs`

### `LOCAL_STORAGE_PATH`

Path where files are stored when using local storage.

- **Default**: `./uploads`
- **Example**: `/var/app/uploads`

### `GCS_BUCKET_NAME`

Google Cloud Storage bucket name (required when `STORAGE_TYPE=gcs`).

- **Default**: `my-bucket`
- **Example**: `alpha3d-uploads`

## Local Storage

```bash
# Use local filesystem (default)
STORAGE_TYPE=local
LOCAL_STORAGE_PATH=./uploads
```

Files are stored at: `./uploads/{uuid}_{filename}`

## GCS Storage (Simulated)

```bash
# Simulate GCS with local fallback
STORAGE_TYPE=gcs
GCS_BUCKET_NAME=my-bucket
```

For development/testing, this creates a local directory structure:
`./uploads/gcs-simulation/{bucket-name}/{uuid}_{filename}`

Returns paths in the format: `gs://{bucket-name}/{filename}`

## Production GCS Integration

To use real Google Cloud Storage:

1. Install `gcloud-storage` crate (alternative to removed `google-cloud-storage`)

2. Set up GCP credentials:

   ```bash
   export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json
   ```

3. Update `src/storage.rs` `GcsStorage` implementation to use actual GCS API

4. Set environment variables:

   ```bash
   STORAGE_TYPE=gcs
   GCS_BUCKET_NAME=your-production-bucket
   ```

## Testing

Acceptance tests automatically use the configured storage backend:

```bash
# Test with local storage
STORAGE_TYPE=local cargo test --test acceptance

# Test with simulated GCS
STORAGE_TYPE=gcs GCS_BUCKET_NAME=test-bucket cargo test --test acceptance
```
