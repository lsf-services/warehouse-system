//! Warehouse Management System - Data Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

// Re-export common types
pub use chrono;
pub use rust_decimal;
pub use validator;

// ============================================================================
// WAREHOUSE MODELS
// ============================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Warehouse {
    pub warehouse_id: i32,
    pub warehouse_code: String,
    pub warehouse_name: String,
    pub warehouse_type: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub manager_user_id: Option<i32>,
    pub timezone: Option<String>,
    pub is_active: bool,
    // Make timestamps nullable to handle database nulls
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateWarehouse {
    #[validate(length(min = 1, max = 50))]
    pub warehouse_code: String,
    #[validate(length(min = 1, max = 255))]
    pub warehouse_name: String,
    pub warehouse_type: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 20))]
    pub phone: Option<String>,
    pub manager_user_id: Option<i32>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateWarehouse {
    #[validate(length(min = 1, max = 255))]
    pub warehouse_name: Option<String>,
    pub warehouse_type: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 20))]
    pub phone: Option<String>,
    pub manager_user_id: Option<i32>,
    pub timezone: Option<String>,
}

// Rest of the models remain the same...

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: Utc::now(),
        }
    }
    
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
            timestamp: Utc::now(),
        }
    }
    
    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            search: None,
            sort_by: None,
            sort_order: Some("ASC".to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, limit: i64) -> Self {
        Self {
            data,
            pagination: PaginationMeta::new(total, page, limit),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl PaginationMeta {
    pub fn new(total: i64, page: i64, limit: i64) -> Self {
        let total_pages = if limit > 0 { (total + limit - 1) / limit } else { 0 };
        Self {
            total,
            page,
            limit,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub services: HealthServices,
    pub uptime: String,
}

#[derive(Debug, Serialize)]
pub struct HealthServices {
    pub database: ServiceHealth,
    pub redis: ServiceHealth,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub status: String,
    pub response_time_ms: Option<u64>,
    pub error: Option<String>,
}

// ============================================================================
// ITEM MODELS (Complete Implementation)
// ============================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Item {
    pub item_id: i32,
    pub item_code: String,
    pub item_name: String,
    pub item_description: Option<String>,
    pub item_type: String,
    pub item_usage_type: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub unit: Option<String>,
    
    // Physical properties
    pub weight_kg: Option<Decimal>,
    pub length_cm: Option<Decimal>,
    pub width_cm: Option<Decimal>,
    pub height_cm: Option<Decimal>,
    pub volume_cbm: Option<Decimal>,
    
    // Tool/Asset specific
    pub is_loanable: bool,
    pub requires_return: bool,
    pub max_loan_duration_days: Option<i32>,
    pub replacement_cost: Option<Decimal>,
    pub maintenance_required: bool,
    pub calibration_required: bool,
    
    // Financial
    pub standard_cost: Option<Decimal>,
    pub last_cost: Option<Decimal>,
    pub average_cost: Option<Decimal>,
    
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateItem {
    #[validate(length(min = 1, max = 100))]
    pub item_code: String,
    #[validate(length(min = 1, max = 255))]
    pub item_name: String,
    pub item_description: Option<String>,
    pub item_type: String,
    pub item_usage_type: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub unit: Option<String>,
    pub is_loanable: Option<bool>,
    pub maintenance_required: Option<bool>,
    pub calibration_required: Option<bool>,
    pub replacement_cost: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateItem {
    #[validate(length(min = 1, max = 255))]
    pub item_name: Option<String>,
    pub item_description: Option<String>,
    pub item_type: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub unit: Option<String>,
    pub replacement_cost: Option<Decimal>,
}

// ============================================================================
// STOCK INVENTORY MODELS
// ============================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StockInventory {
    pub stock_id: i32,
    pub item_id: i32,
    pub warehouse_id: i32,
    pub quantity_on_hand: Decimal,
    pub quantity_reserved: Decimal,
    pub quantity_available: Option<Decimal>,
    pub min_stock_level: Option<Decimal>,
    pub max_stock_level: Option<Decimal>,
    pub reorder_point: Option<Decimal>,
    pub unit_cost: Option<Decimal>,
    pub average_cost: Option<Decimal>,
    pub total_value: Option<Decimal>,
    pub last_movement_date: Option<NaiveDate>,
    pub last_receipt_date: Option<NaiveDate>,
    pub last_issue_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemWithStock {
    #[serde(flatten)]
    pub item: Item,
    pub stock_info: Vec<StockInventory>,
}
