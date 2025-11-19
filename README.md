# Alpha3D

A minimal full-stack template that pairs a Vue 3 + Vite frontend with an Axum backend. OpenAPI documentation is generated automatically via [Utoipa](https://crates.io/crates/utoipa) and served with Swagger UI so you can explore the API immediately.

## Project layout

```txt
.
├── Cargo.toml
├── src/
│   └── main.rs          # Axum server + OpenAPI doc generation
├── Dockerfile.backend   # Multi-stage build for the Axum API
├── docker-compose.yml   # Spins up backend + frontend together
├── frontend/
│   ├── package.json     # Vue + Vite project
│   ├── Dockerfile       # Builds the static Vue bundle + nginx image
│   └── nginx.conf       # Proxies /api requests to the backend
│   └── src/
│       ├── App.vue
│       └── components/
│           └── GreetingCard.vue
└── README.md
```

## Requirements

- Rust (1.80+) with `cargo`
- Node.js 18+ with npm
- Docker 24+ (optional, for containerized workflow)
  - The Docker builder image pins `rustlang/rust:nightly` so Cargo can compile the Edition 2024 crate.
- gcloud CLI (optional, for GCP deployment)

## Backend (Axum + Utoipa)

```bash
# from repo root
cargo run
```

- Serves REST endpoints at `http://localhost:3000`
- Swagger UI + OpenAPI JSON available at `http://localhost:3000/docs`
- Sample routes
  - `GET /api/greeting`
  - `POST /api/echo`

## Frontend (Vue + Vite)

```bash
cd frontend
npm install
npm run dev
```

- Dev server runs at `http://localhost:5173`
- Requests to `/api/*` are proxied to the Axum server, so both apps can run concurrently

For a production build:

```bash
cd frontend
npm run build
```

## Recommended workflow

1. Start the Rust API with `cargo run`
2. Start the Vue dev server with `npm run dev` inside `frontend`
3. Visit the UI at `http://localhost:5173` and the API docs at `http://localhost:3000/docs`

## Docker Compose workflow

Build and run both services with one command:

```bash
docker compose up --build
```

- Backend is available at `http://localhost:3000`
- Frontend (Nginx) serves the built assets at `http://localhost:5173`
- The Nginx config proxies `/api`, `/docs`, and `/api-doc` requests to the backend container so the SPA works without extra environment wiring.

To tear everything down:

```bash
docker compose down
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

- [ ] **File Upload**: Multipart upload endpoint (STL/OBJ)
- [ ] **Cloud Storage**: Google Cloud Storage integration
- [ ] **Geometry Analysis**: Volume, Surface Area, Bounding Box calculation (Rust)
- [x] **Database Schema**: Files table (Schema created, Logic pending)

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
