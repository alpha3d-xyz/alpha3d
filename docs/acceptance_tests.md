# Acceptance Tests

This project includes acceptance tests to verify the backend endpoints. These tests are written in Rust using `reqwest` and `tokio` and can be run against a local server or a deployed instance (e.g., on Google Cloud Platform).

## Prerequisites

- Rust (1.80+)
- A running instance of the backend application

## Running Tests Locally

1. **Start the backend server:**

   Open a terminal and run:

   ```bash
   cargo run
   ```

   The server will start at `http://localhost:3000`.

2. **Run the tests:**

   Open a separate terminal and run:

   ```bash
   cargo test --test acceptance
   ```

   By default, the tests target `http://localhost:3000`.

## Running Tests Against GCP (or other remote environments)

You can configure the target URL using the `TEST_BASE_URL` environment variable. This allows you to run the acceptance tests against a deployed version of the application.

1. **Deploy your application** (e.g., to Cloud Run).
2. **Run the tests with the environment variable:**

   ```bash
   TEST_BASE_URL=https://your-gcp-app-url.a.run.app cargo test --test acceptance
   ```

   Replace `https://your-gcp-app-url.a.run.app` with your actual service URL.

## Test Coverage

The acceptance tests currently cover the following scenarios defined in the [Technical Specification](TECHNICAL_SPEC.md):

### Core Workflows

1.  **User Signup Flow**
    *   `POST /api/auth/signup`: Register a new user.
    *   `POST /api/auth/login`: Authenticate and receive a JWT.
    *   `GET /api/auth/me`: Verify profile retrieval with the token.

2.  **Quote Workflow**
    *   `POST /api/files/upload`: Upload a sample STL file.
    *   `GET /api/files/:id/analysis`: Poll for analysis completion (Volume/Area).
    *   `POST /api/quotes/calculate`: Request a price for specific material options.

3.  **Order Lifecycle**
    *   `POST /api/orders`: Create an order from a valid quote.
    *   `GET /api/orders`: Verify the order appears in the user's list.
    *   `PATCH /api/admin/orders/:id/status`: (Admin) Update status to 'PRINTING'.

### Legacy Tests (Smoke Tests)

- `GET /api/greeting`: Verifies that the endpoint returns a 200 OK status.
- `POST /api/echo`: Verifies that the endpoint echoes back the payload.
