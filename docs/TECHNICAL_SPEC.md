# Alpha3D Technical Specification & Implementation Guide

This document provides detailed instructions for AI agents and developers to implement the Alpha3D platform. It breaks down the high-level requirements into specific coding tasks, data models, API contracts, and test specifications.

## 1. Database Schema (PostgreSQL)

The backend will use `sqlx` with PostgreSQL.

### 1.1. Users Table
- **id**: UUID (Primary Key)
- **email**: VARCHAR (Unique, Not Null)
- **password_hash**: VARCHAR (Argon2, Not Null)
- **role**: VARCHAR (ENUM: 'USER', 'ADMIN')
- **created_at**: TIMESTAMPTZ
- **updated_at**: TIMESTAMPTZ

### 1.2. Files Table
- **id**: UUID (Primary Key)
- **user_id**: UUID (Foreign Key -> Users.id)
- **filename**: VARCHAR
- **gcs_path**: VARCHAR (Path in Google Cloud Storage)
- **file_size_bytes**: BIGINT
- **volume_cm3**: DOUBLE PRECISION (Calculated)
- **surface_area_cm2**: DOUBLE PRECISION (Calculated)
- **status**: VARCHAR (ENUM: 'PENDING', 'ANALYZED', 'FAILED')
- **created_at**: TIMESTAMPTZ

### 1.3. Quotes Table
- **id**: UUID (Primary Key)
- **file_id**: UUID (Foreign Key -> Files.id)
- **material**: VARCHAR (ENUM: 'PLA', 'ABS', 'RESIN')
- **color**: VARCHAR
- **layer_height**: DOUBLE PRECISION (mm)
- **infill_percentage**: INTEGER
- **estimated_cost**: DECIMAL (KRW)
- **created_at**: TIMESTAMPTZ

### 1.4. Orders Table
- **id**: UUID (Primary Key)
- **user_id**: UUID (Foreign Key -> Users.id)
- **quote_id**: UUID (Foreign Key -> Quotes.id)
- **status**: VARCHAR (ENUM: 'PAID', 'PRINTING', 'SHIPPED', 'DELIVERED')
- **shipping_address**: JSONB
- **tracking_number**: VARCHAR
- **created_at**: TIMESTAMPTZ

---

## 2. Backend Implementation (Rust/Axum)

### 2.1. Authentication Module
*   **Task**: Implement JWT-based auth with Argon2 password hashing.
*   **Endpoints**:
    *   `POST /api/auth/signup`: Register new user.
    *   `POST /api/auth/login`: Return JWT token.
    *   `GET /api/auth/me`: Return current user profile (Protected).
*   **Unit Tests**:
    *   `test_password_hashing`: Verify Argon2 hashing/verification.
    *   `test_jwt_generation`: Verify token creation and expiration.

### 2.2. File Upload & Analysis Module (Core Engine)
*   **Task**: Handle multipart uploads, stream to GCS, and trigger geometry analysis.
*   **Endpoints**:
    *   `POST /api/files/upload`: Multipart form data (STL/OBJ). Returns `file_id`.
    *   `GET /api/files/:id/analysis`: Return volume, area, and bounding box.
*   **Core Logic (Rust)**:
    *   Use `stl_io` or `parry3d` to parse mesh.
    *   Calculate Volume: Signed tetrahedron volume summation.
    *   Calculate Surface Area: Sum of triangle areas.
*   **Unit Tests**:
    *   `test_volume_calculation_cube`: Parse a 10x10x10mm cube STL and assert volume is ~1000mm³.
    *   `test_surface_area_cube`: Assert area is ~600mm².
    *   `test_invalid_stl`: Handle non-manifold or corrupted files gracefully.

### 2.3. Quoting Engine
*   **Task**: Calculate price based on analysis data and material config.
*   **Logic**: `Price = (Volume * Material_Density * Material_Cost_Per_Gram) + (Print_Time_Est * Machine_Hourly_Rate) + Markup`.
*   **Endpoints**:
    *   `POST /api/quotes/calculate`: Input `{ file_id, material, options }`, Output `{ cost, breakdown }`.
*   **Unit Tests**:
    *   `test_pricing_logic`: Verify cost calculation formula with fixed inputs.

### 2.4. Order Management
*   **Task**: Create orders and mock payment processing.
*   **Endpoints**:
    *   `POST /api/orders`: Create order from Quote ID.
    *   `GET /api/orders`: List user's orders.
    *   `PATCH /api/admin/orders/:id/status`: Admin updates status.

---

## 3. Frontend Implementation (Vue 3)

### 3.1. 3D Viewer Component (`ThreeViewer.vue`)
*   **Task**: Render STL/OBJ files using Three.js.
*   **Features**: Auto-rotate, Zoom, Pan, "Fit to Screen".
*   **Props**: `fileUrl` (Blob URL or GCS signed URL).

### 3.2. Upload & Quote Page
*   **Task**: Drag & Drop zone -> Upload API -> Poll Analysis -> Show Viewer & Price.
*   **State**: Use Pinia to store current file analysis and quote configuration.

### 3.3. Admin Dashboard
*   **Task**: Table view of all orders with Status dropdowns.

---

## 4. Testing Specification

### 4.1. Unit Tests (Rust)
Run with `cargo test`.

| Module | Test Case | Description |
| :--- | :--- | :--- |
| **Engine** | `calc_cube_volume` | Validate volume math on a unit cube. |
| **Engine** | `calc_sphere_volume` | Validate volume math on a unit sphere. |
| **Auth** | `hash_password` | Ensure password is not stored in plain text. |
| **Quote** | `material_cost_pla` | Verify PLA cost multiplier is applied correctly. |

### 4.2. Acceptance Tests (End-to-End)
Run with `cargo test --test acceptance` (requires running DB/Server).

| Scenario | Steps | Expected Result |
| :--- | :--- | :--- |
| **User Signup Flow** | 1. POST /signup <br> 2. POST /login | Receive valid JWT token. |
| **Quote Workflow** | 1. Upload 'cube.stl' <br> 2. GET /analysis <br> 3. POST /quote | 1. File ID returned. <br> 2. Volume ~1000. <br> 3. Price > 0. |
| **Order Lifecycle** | 1. Create Order <br> 2. Admin updates status | User sees status change to 'PRINTING'. |

### 4.3. Frontend Component Tests (Vitest)
Run with `npm run test:unit`.

*   **Viewer**: Mount `ThreeViewer` and check if canvas is rendered.
*   **QuoteForm**: Check if changing material dropdown updates the displayed price (mocked API).

