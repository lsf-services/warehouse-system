-- PostgreSQL initialization script
-- This runs when the container starts for the first time

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gin";

-- Create schemas
CREATE SCHEMA IF NOT EXISTS warehouse;
CREATE SCHEMA IF NOT EXISTS audit;
CREATE SCHEMA IF NOT EXISTS analytics;

-- Set default search path
ALTER DATABASE warehouse_db SET search_path TO warehouse, public;

-- Create development user with appropriate permissions
DO $$
BEGIN
    -- Grant permissions to warehouse_user
    GRANT ALL PRIVILEGES ON DATABASE warehouse_db TO warehouse_user;
    GRANT ALL PRIVILEGES ON SCHEMA warehouse TO warehouse_user;
    GRANT ALL PRIVILEGES ON SCHEMA audit TO warehouse_user;
    GRANT ALL PRIVILEGES ON SCHEMA analytics TO warehouse_user;
    
    -- Set default privileges for future objects
    ALTER DEFAULT PRIVILEGES IN SCHEMA warehouse GRANT ALL ON TABLES TO warehouse_user;
    ALTER DEFAULT PRIVILEGES IN SCHEMA warehouse GRANT ALL ON SEQUENCES TO warehouse_user;
    ALTER DEFAULT PRIVILEGES IN SCHEMA warehouse GRANT ALL ON FUNCTIONS TO warehouse_user;
END
$$;

-- Create test database for development
CREATE DATABASE warehouse_test WITH OWNER warehouse_user;

-- Connect to test database and setup extensions
\c warehouse_test;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gin";
CREATE SCHEMA IF NOT EXISTS warehouse;
ALTER DATABASE warehouse_test SET search_path TO warehouse, public;

-- Switch back to main database
\c warehouse_db;

-- Log successful initialization
INSERT INTO pg_stat_statements_reset();
SELECT 'PostgreSQL initialization completed successfully' as status;
