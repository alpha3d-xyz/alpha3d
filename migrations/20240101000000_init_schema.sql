-- Create Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'USER',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Files Table
CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    filename VARCHAR(255) NOT NULL,
    gcs_path VARCHAR(255) NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    volume_cm3 DOUBLE PRECISION,
    surface_area_cm2 DOUBLE PRECISION,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Quotes Table
CREATE TABLE quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL REFERENCES files(id),
    material VARCHAR(50) NOT NULL,
    color VARCHAR(50) NOT NULL,
    layer_height DOUBLE PRECISION NOT NULL,
    infill_percentage INTEGER NOT NULL,
    estimated_cost DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Orders Table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    quote_id UUID NOT NULL REFERENCES quotes(id),
    status VARCHAR(50) NOT NULL DEFAULT 'PAID',
    shipping_address JSONB NOT NULL,
    tracking_number VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
