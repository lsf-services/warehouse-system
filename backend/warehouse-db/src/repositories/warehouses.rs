//! Warehouse repository implementation

use anyhow::Result;
use sqlx::{PgPool, Row}; // Add Row trait import
use warehouse_models::*;
use crate::utils::*;

#[derive(Clone)]
pub struct WarehouseRepository {
    pool: PgPool,
}

impl WarehouseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new warehouse - using query_as! macro with proper field mapping
    pub async fn create(&self, warehouse: CreateWarehouse) -> Result<Warehouse> {
        let result = sqlx::query!(
            r#"
            INSERT INTO warehouse.warehouses (
                warehouse_code, warehouse_name, warehouse_type,
                address, city, state, postal_code, country, phone, email,
                manager_user_id, timezone, created_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING 
                warehouse_id, warehouse_code, warehouse_name, warehouse_type,
                address, city, state, postal_code, country, phone, email,
                manager_user_id, timezone, is_active, 
                created_at, updated_at, created_by, updated_by
            "#,
            warehouse.warehouse_code,
            warehouse.warehouse_name,
            warehouse.warehouse_type,
            warehouse.address,
            warehouse.city,
            warehouse.state,
            warehouse.postal_code,
            warehouse.country.unwrap_or_else(|| "Indonesia".to_string()),
            warehouse.phone,
            warehouse.email,
            warehouse.manager_user_id,
            warehouse.timezone.unwrap_or_else(|| "Asia/Jakarta".to_string()),
            1i32 // Default created_by
        )
        .fetch_one(&self.pool)
        .await?;

        // Convert to our struct
        let warehouse = Warehouse {
            warehouse_id: result.warehouse_id,
            warehouse_code: result.warehouse_code,
            warehouse_name: result.warehouse_name,
            warehouse_type: result.warehouse_type,
            address: result.address,
            city: result.city,
            state: result.state,
            postal_code: result.postal_code,
            country: result.country,
            phone: result.phone,
            email: result.email,
            manager_user_id: result.manager_user_id,
            timezone: result.timezone,
            is_active: result.is_active,
            created_at: result.created_at,
            updated_at: result.updated_at,
            created_by: result.created_by,
            updated_by: result.updated_by,
        };

        Ok(warehouse)
    }

    /// Get warehouse by ID - using query! macro
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Warehouse>> {
        let result = sqlx::query!(
            "SELECT * FROM warehouse.warehouses WHERE warehouse_id = $1 AND is_active = true",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let warehouse = Warehouse {
                    warehouse_id: row.warehouse_id,
                    warehouse_code: row.warehouse_code,
                    warehouse_name: row.warehouse_name,
                    warehouse_type: row.warehouse_type,
                    address: row.address,
                    city: row.city,
                    state: row.state,
                    postal_code: row.postal_code,
                    country: row.country,
                    phone: row.phone,
                    email: row.email,
                    manager_user_id: row.manager_user_id,
                    timezone: row.timezone,
                    is_active: row.is_active,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    created_by: row.created_by,
                    updated_by: row.updated_by,
                };
                Ok(Some(warehouse))
            }
            None => Ok(None),
        }
    }

    /// Get warehouse by code
    pub async fn get_by_code(&self, code: &str) -> Result<Option<Warehouse>> {
        let result = sqlx::query!(
            "SELECT * FROM warehouse.warehouses WHERE warehouse_code = $1 AND is_active = true",
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let warehouse = Warehouse {
                    warehouse_id: row.warehouse_id,
                    warehouse_code: row.warehouse_code,
                    warehouse_name: row.warehouse_name,
                    warehouse_type: row.warehouse_type,
                    address: row.address,
                    city: row.city,
                    state: row.state,
                    postal_code: row.postal_code,
                    country: row.country,
                    phone: row.phone,
                    email: row.email,
                    manager_user_id: row.manager_user_id,
                    timezone: row.timezone,
                    is_active: row.is_active,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    created_by: row.created_by,
                    updated_by: row.updated_by,
                };
                Ok(Some(warehouse))
            }
            None => Ok(None),
        }
    }

    /// List warehouses with pagination - simplified version
    pub async fn list(&self, pagination: PaginationQuery) -> Result<PaginatedResponse<Warehouse>> {
        let (page, limit) = validate_pagination(&pagination);
        let offset = calculate_offset(page, limit);

        // Get total count
        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM warehouse.warehouses WHERE is_active = true"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        // Get data
        let rows = sqlx::query!(
            "SELECT * FROM warehouse.warehouses WHERE is_active = true 
             ORDER BY warehouse_name LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let mut warehouses = Vec::new();
        for row in rows {
            let warehouse = Warehouse {
                warehouse_id: row.warehouse_id,
                warehouse_code: row.warehouse_code,
                warehouse_name: row.warehouse_name,
                warehouse_type: row.warehouse_type,
                address: row.address,
                city: row.city,
                state: row.state,
                postal_code: row.postal_code,
                country: row.country,
                phone: row.phone,
                email: row.email,
                manager_user_id: row.manager_user_id,
                timezone: row.timezone,
                is_active: row.is_active,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: row.created_by,
                updated_by: row.updated_by,
            };
            warehouses.push(warehouse);
        }

        Ok(PaginatedResponse::new(warehouses, total, page, limit))
    }

    /// Update warehouse - simplified version
    pub async fn update(&self, id: i32, warehouse: UpdateWarehouse) -> Result<Option<Warehouse>> {
        let result = sqlx::query!(
            r#"
            UPDATE warehouse.warehouses SET 
                warehouse_name = COALESCE($2, warehouse_name),
                warehouse_type = COALESCE($3, warehouse_type),
                address = COALESCE($4, address),
                city = COALESCE($5, city),
                state = COALESCE($6, state),
                country = COALESCE($7, country),
                email = COALESCE($8, email),
                phone = COALESCE($9, phone),
                updated_at = NOW(),
                updated_by = $10
            WHERE warehouse_id = $1 AND is_active = true
            RETURNING *
            "#,
            id,
            warehouse.warehouse_name,
            warehouse.warehouse_type,
            warehouse.address,
            warehouse.city,
            warehouse.state,
            warehouse.country,
            warehouse.email,
            warehouse.phone,
            Some(1i32) // updated_by
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let warehouse = Warehouse {
                    warehouse_id: row.warehouse_id,
                    warehouse_code: row.warehouse_code,
                    warehouse_name: row.warehouse_name,
                    warehouse_type: row.warehouse_type,
                    address: row.address,
                    city: row.city,
                    state: row.state,
                    postal_code: row.postal_code,
                    country: row.country,
                    phone: row.phone,
                    email: row.email,
                    manager_user_id: row.manager_user_id,
                    timezone: row.timezone,
                    is_active: row.is_active,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    created_by: row.created_by,
                    updated_by: row.updated_by,
                };
                Ok(Some(warehouse))
            }
            None => Ok(None),
        }
    }

    /// Soft delete warehouse
    pub async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query!(
            "UPDATE warehouse.warehouses SET is_active = false, updated_at = NOW(), updated_by = $2 
             WHERE warehouse_id = $1 AND is_active = true",
            id,
            1i32 // updated_by
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Check if warehouse code exists
    pub async fn code_exists(&self, code: &str, exclude_id: Option<i32>) -> Result<bool> {
        let exists = match exclude_id {
            Some(id) => {
                sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM warehouse.warehouses 
                     WHERE warehouse_code = $1 AND warehouse_id != $2 AND is_active = true)",
                    code, 
                    id
                )
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM warehouse.warehouses 
                     WHERE warehouse_code = $1 AND is_active = true)",
                    code
                )
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(exists.unwrap_or(false))
    }
}
