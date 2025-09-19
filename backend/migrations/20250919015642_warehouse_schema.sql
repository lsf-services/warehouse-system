-- Warehouse Management System - Clean Schema

-- Enable extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create warehouse schema
CREATE SCHEMA IF NOT EXISTS warehouse;
SET search_path TO warehouse, public;

-- Simple warehouses table
CREATE TABLE warehouses (
    warehouse_id SERIAL PRIMARY KEY,
    warehouse_code VARCHAR(50) UNIQUE NOT NULL,
    warehouse_name VARCHAR(255) NOT NULL,
    warehouse_type VARCHAR(50) DEFAULT 'STANDARD',
    address TEXT,
    city VARCHAR(100),
    state VARCHAR(100),
    postal_code VARCHAR(20),
    country VARCHAR(100) DEFAULT 'Indonesia',
    phone VARCHAR(20),
    email VARCHAR(100),
    manager_user_id INTEGER,
    timezone VARCHAR(50) DEFAULT 'Asia/Jakarta',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by INTEGER,
    updated_by INTEGER
);

-- Insert sample data
INSERT INTO warehouses (warehouse_code, warehouse_name, city, state, created_by, updated_by) VALUES
('WH001', 'Jakarta Main Warehouse', 'Jakarta', 'DKI Jakarta', 1, 1),
('WH002', 'Surabaya Branch', 'Surabaya', 'East Java', 1, 1);
