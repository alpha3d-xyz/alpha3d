# Alpha3D

**Alpha3D** is a next-generation 3D prototyping platform designed for design students and early-stage startups. It provides instant AI-based quoting and 3D printing ordering services, leveraging Rust's high performance for real-time model analysis.

## Key Features

- **Instant Quote**: Real-time price calculation based on volume, material, and print time.
- **3D Model Analysis**: Automatic calculation of volume, surface area, and bounding box for STL/OBJ files.
- **Smart Viewer**: Web-based 3D viewer with auto-rotation and size visualization.
- **Secure Auth**: JWT-based authentication with Argon2 password hashing.
- **Cloud Native**: Designed for Google Cloud Platform (Cloud Run, Cloud SQL, Cloud Storage).

## Documentation

- [Technical Specification](docs/TECHNICAL_SPEC.md)
- [Git Branch Strategy](docs/git-branch-strategy.md)
- [Request for Specification](docs/RFS.md)

## Project layout

```txt
.
├── Cargo.toml
├── src/                 # Rust Backend (Axum)
│   ├── main.rs
│   ├── lib.rs
│   ├── quoting.rs       # Quoting Logic
│   ├── analysis.rs      # Geometry Analysis
│   ├── handlers/        # API Handlers
│   └── models.rs        # Database Models
├── tests/               # Integration & Acceptance Tests
├── frontend/            # Vue 3 + Vite Frontend
├── migrations/          # SQLx Database Migrations
├── docs/                # Project Documentation
└── docker-compose.yml   # Local Development Environment
```

## Requirements

- Rust (1.80+) with `cargo`
- Node.js 18+ with npm
- Docker 24+ (optional, for containerized workflow)
- PostgreSQL 15+ (if running locally without Docker)

## Getting Started

### 1. Database Setup

```bash
# Start Postgres via Docker
docker compose up -d db

# Run Migrations
sqlx migrate run
```

### 2. Backend (Axum)

```bash
# Run locally
cargo run

# Run Tests
cargo test
```

- API: `http://localhost:3000`
- Swagger UI: `http://localhost:3000/swagger-ui`

### 3. Frontend (Vue + Vite)

```bash
cd frontend
npm install
npm run dev
```

- UI: `http://localhost:5173`

## Docker Compose workflow

Build and run both services with one command:

```bash
docker compose up --build
```

## Deploying to Google Cloud Run

This repo ships with everything you need to run both services on Cloud Run:

- `deploy/gcp/cloudbuild.yaml` – Cloud Build pipeline that builds/pushes the backend & frontend images, then deploys them to Cloud Run. Override `_BACKEND_BASE_URL` with the backend service URL (no trailing slash) so the nginx proxy can forward `/api`, `/docs`, and `/api-doc` requests from the SPA.
- `deploy/gcp/cloudrun/` – Terraform module that provisions two Cloud Run services (frontend + backend) once images exist in Artifact Registry.

Quick start after authenticating `gcloud` and creating an Artifact Registry repository:

```bash
BACKEND_URL="https://alpha3d-backend-xxxxx-asia-northeast3.a.run.app"
gcloud builds submit \
  --config deploy/gcp/cloudbuild.yaml \
  --substitutions=_REGION=asia-northeast3,_REPOSITORY=alpha3d,_BACKEND_BASE_URL=$BACKEND_URL
```

See `deploy/gcp/README.md` for the full step-by-step guide, including Terraform usage.

## Feature Checklist

### Core Infrastructure

- [x] **Project Setup**: Vue 3 + Vite (Frontend), Axum (Backend)
- [x] **Dockerization**: Dockerfile for Backend & Frontend, docker-compose.yml
- [x] **Database**: PostgreSQL integration with `sqlx`
- [x] **Deployment**: Google Cloud Run configuration (Cloud Build, Terraform)

### Authentication & User Management

- [x] **Backend Auth**: Signup, Login, JWT Token generation, Argon2 hashing
- [x] **Middleware**: Protected routes verification
- [x] **Frontend Auth**: Login/Signup forms, Pinia Store, Router Guards
- [x] **Database Schema**: Users table

### File Management & Analysis (Core Engine)

- [x] **File Upload**: Multipart upload endpoint with 100MB size limit
- [x] **Storage Abstraction**: Pluggable storage backend (Local/GCS)
- [x] **Local Storage**: Development/testing with local filesystem
- [x] **GCS Support**: Simulated GCS storage (ready for production integration)
- [x] **Geometry Analysis**: Volume, Surface Area calculation from STL files
- [x] **Database Schema**: Files table with analysis metadata

### Quoting System

- [ ] **Pricing Logic**: Material cost + Machine time + Markup
- [ ] **Quote Generation**: API to calculate price based on analysis
- [x] **Database Schema**: Quotes table (Schema created, Logic pending)

### Order Management

- [ ] **Order Creation**: Convert Quote to Order
- [ ] **Order Status**: Tracking (Paid, Printing, Shipped)
- [x] **Database Schema**: Orders table (Schema created, Logic pending)

### Frontend Features

- [ ] **3D Viewer**: Three.js integration for STL preview
- [ ] **Upload Interface**: Drag & drop file upload
- [ ] **Quote Display**: Real-time price estimation
- [ ] **Order History**: User dashboard

## Acceptance Tests

See [docs/acceptance_tests.md](docs/acceptance_tests.md) for instructions on how to run acceptance tests locally and against GCP.
