# GCP deployment guide

Two options are provided to run the starter on Google Cloud Platform:

1. **Cloud Build + Cloud Run (YAML pipeline)** – builds and deploys both containers from Git tags or commits.
2. **Terraform (cloudrun/)** – declaratively provisions the Cloud Run services once you have container images available in Artifact Registry.

## Prerequisites

- `gcloud` CLI authenticated against your project (`gcloud auth application-default login`).
- Artifact Registry repository for the images (e.g., `REGION-docker.pkg.dev/PROJECT/alpha3d`).
- Enabled APIs: `run.googleapis.com`, `artifactregistry.googleapis.com`, `cloudbuild.googleapis.com`.

## Cloud Build workflow

1. Replace the substitution defaults at the top of `deploy/gcp/cloudbuild.yaml` or override them when launching a build:
   - `_REGION`: Cloud Run region (e.g., `asia-northeast3`).
   - `_REPOSITORY`: Artifact Registry repository name.
   - `_BACKEND_SERVICE` / `_FRONTEND_SERVICE`: Cloud Run service names.
   - `_BACKEND_BASE_URL`: Public HTTPS URL of the backend service (after the first deployment, update this value so the frontend proxy forwards requests correctly). No trailing slash.
2. Run the build from the repo root:

```bash
PROJECT_ID="your-project"
BACKEND_URL="https://alpha3d-backend-xxxxx-asia-northeast3.a.run.app"
gcloud builds submit \
  --project "$PROJECT_ID" \
  --config deploy/gcp/cloudbuild.yaml \
  --substitutions=_REGION=asia-northeast3,_REPOSITORY=alpha3d,_BACKEND_BASE_URL=$BACKEND_URL
```

The pipeline builds & pushes both images, then deploys two Cloud Run services with unauthenticated access enabled. The frontend service receives the `BACKEND_BASE_URL` env var so its Nginx proxy can forward SPA requests to the backend.

## Terraform workflow

1. Build & push images (via Cloud Build above or manually) so you have immutable tags to reference.
2. Copy `deploy/gcp/cloudrun/terraform.tfvars.example` to `terraform.tfvars` and fill in your project, region, and image URLs.
3. Initialize and apply:

```bash
cd deploy/gcp/cloudrun
terraform init
terraform apply
```

Terraform outputs the public URLs for both services. Update the frontend Cloud Run service (or re-run the Cloud Build pipeline) with `BACKEND_BASE_URL` set to the backend URL so the reverse proxy can talk to the API.
