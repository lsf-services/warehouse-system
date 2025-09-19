-- Items Management Schema

-- Items table
CREATE TABLE warehouse.items (
    item_id SERIAL PRIMARY KEY,
    item_code VARCHAR(100) UNIQUE NOT NULL,
    item_name VARCHAR(255) NOT NULL,
    item_description TEXT,
    item_type VARCHAR(50) NOT NULL DEFAULT 'STOCK',
    item_usage_type VARCHAR(50) DEFAULT 'CONSUMABLE',
    category VARCHAR(100),
    subcategory VARCHAR(100),
    brand VARCHAR(100),
    model VARCHAR(100),
    unit VARCHAR(50) DEFAULT 'PCS',
    
    -- Physical properties
    weight_kg DECIMAL(10,4),
    length_cm DECIMAL(8,2),
    width_cm DECIMAL(8,2),
    height_cm DECIMAL(8,2),
    volume_cbm DECIMAL(10,4),
    
    -- Tool/Asset specific
    is_loanable BOOLEAN DEFAULT FALSE,
    requires_return BOOLEAN DEFAULT FALSE,
    max_loan_duration_days INTEGER DEFAULT 30,
    replacement_cost DECIMAL(15,4),
    maintenance_required BOOLEAN DEFAULT FALSE,
    calibration_required BOOLEAN DEFAULT FALSE,
    
    -- Financial
    standard_cost DECIMAL(15,4),
    last_cost DECIMAL(15,4),
    average_cost DECIMAL(15,4),
    
    status VARCHAR(20) DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by INTEGER,
    updated_by INTEGER
);

-- Stock inventory table
CREATE TABLE warehouse.stock_inventory (
    stock_id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL REFERENCES warehouse.items(item_id) ON DELETE CASCADE,
    warehouse_id INTEGER NOT NULL REFERENCES warehouse.warehouses(warehouse_id) ON DELETE CASCADE,
    
    -- Stock quantities
    quantity_on_hand DECIMAL(15,4) NOT NULL DEFAULT 0 CHECK (quantity_on_hand >= 0),
    quantity_reserved DECIMAL(15,4) NOT NULL DEFAULT 0 CHECK (quantity_reserved >= 0),
    quantity_available DECIMAL(15,4) GENERATED ALWAYS AS (quantity_on_hand - quantity_reserved) STORED,
    
    -- Planning parameters
    min_stock_level DECIMAL(15,4) DEFAULT 0,
    max_stock_level DECIMAL(15,4) DEFAULT 0,
    reorder_point DECIMAL(15,4) DEFAULT 0,
    
    -- Cost tracking
    unit_cost DECIMAL(15,4),
    average_cost DECIMAL(15,4),
    total_value DECIMAL(18,4) GENERATED ALWAYS AS (quantity_on_hand * COALESCE(average_cost, unit_cost, 0)) STORED,
    
    -- Activity tracking
    last_movement_date DATE,
    last_receipt_date DATE,
    last_issue_date DATE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(item_id, warehouse_id),
    CHECK (quantity_reserved <= quantity_on_hand)
);

-- Indexes for performance
CREATE INDEX idx_items_code_active ON warehouse.items(item_code) WHERE status = 'ACTIVE';
CREATE INDEX idx_items_category_type ON warehouse.items(category, item_type);
CREATE INDEX idx_items_loanable ON warehouse.items(is_loanable) WHERE is_loanable = TRUE;

CREATE INDEX idx_stock_item_warehouse ON warehouse.stock_inventory(item_id, warehouse_id);
CREATE INDEX idx_stock_low_stock ON warehouse.stock_inventory(warehouse_id, reorder_point, quantity_available) 
    WHERE quantity_available <= reorder_point;

-- Sample items data
INSERT INTO warehouse.items (item_code, item_name, item_description, item_type, category, unit, is_loanable, status, created_by, updated_by) VALUES
('ITM001', 'Laptop Dell Inspiron 15', 'Dell Inspiron 15 3000 Series', 'ASSET', 'Electronics', 'PCS', TRUE, 'ACTIVE', 1, 1),
('ITM002', 'Steel Rebar 12mm', 'Steel reinforcement bar 12mm diameter', 'STOCK', 'Construction Materials', 'PCS', FALSE, 'ACTIVE', 1, 1),
('ITM003', 'Safety Helmet', 'Construction safety helmet', 'STOCK', 'Safety Equipment', 'PCS', FALSE, 'ACTIVE', 1, 1),
('ITM004', 'Concrete Mixer 500L', 'Heavy duty concrete mixer', 'ASSET', 'Construction Equipment', 'PCS', TRUE, 'ACTIVE', 1, 1);

-- Sample stock data
INSERT INTO warehouse.stock_inventory (item_id, warehouse_id, quantity_on_hand, quantity_reserved, unit_cost, average_cost, reorder_point) VALUES
(1, 1, 5, 1, 15000000, 15000000, 2),
(2, 1, 1000, 100, 25000, 25000, 200),
(3, 1, 50, 0, 150000, 150000, 10),
(4, 2, 2, 0, 75000000, 75000000, 1);
