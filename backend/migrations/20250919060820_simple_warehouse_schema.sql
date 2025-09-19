-- Simple warehouse schema
CREATE SCHEMA IF NOT EXISTS warehouse;

CREATE TABLE warehouse.warehouses (
    warehouse_id SERIAL PRIMARY KEY,
    warehouse_code VARCHAR(50) UNIQUE NOT NULL,
    warehouse_name VARCHAR(255) NOT NULL,
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100) DEFAULT 'Indonesia',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

INSERT INTO warehouse.warehouses (warehouse_code, warehouse_name, city, state) VALUES
('WH001', 'Jakarta Warehouse', 'Jakarta', 'DKI Jakarta'),
('WH002', 'Surabaya Warehouse', 'Surabaya', 'East Java');
