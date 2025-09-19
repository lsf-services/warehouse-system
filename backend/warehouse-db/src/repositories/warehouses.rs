use anyhow::Result;
use sqlx::PgPool;
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

    pub async fn list(&self, pagination: PaginationQuery) -> Result<PaginatedResponse<Warehouse>> {
        let (page, limit) = validate_pagination(&pagination);
        let offset = calculate_offset(page, limit);

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM warehouse.warehouses WHERE is_active = true"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        let rows = sqlx::query!(
            "SELECT warehouse_id, warehouse_code, warehouse_name, 
                    city, state, country, is_active, created_at, updated_at
             FROM warehouse.warehouses WHERE is_active = true 
             ORDER BY warehouse_name LIMIT $1 OFFSET $2",
            limit, offset
        )
        .fetch_all(&self.pool)
        .await?;

        let mut warehouses = Vec::new();
        for row in rows {
            let warehouse = Warehouse {
                warehouse_id: row.warehouse_id,
                warehouse_code: row.warehouse_code,
                warehouse_name: row.warehouse_name,
                warehouse_type: None,
                address: None,
                city: row.city,
                state: row.state,
                postal_code: None,
                country: row.country,
                phone: None,
                email: None,
                manager_user_id: None,
                timezone: None,
                is_active: row.is_active.unwrap_or(true),
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: None,
                updated_by: None,
            };
            warehouses.push(warehouse);
        }

        Ok(PaginatedResponse::new(warehouses, total, page, limit))
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Warehouse>> {
        let result = sqlx::query!(
            "SELECT warehouse_id, warehouse_code, warehouse_name, 
                    city, state, country, is_active, created_at, updated_at
             FROM warehouse.warehouses WHERE warehouse_id = $1 AND is_active = true",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Warehouse {
                warehouse_id: row.warehouse_id,
                warehouse_code: row.warehouse_code,
                warehouse_name: row.warehouse_name,
                warehouse_type: None,
                address: None,
                city: row.city,
                state: row.state,
                postal_code: None,
                country: row.country,
                phone: None,
                email: None,
                manager_user_id: None,
                timezone: None,
                is_active: row.is_active.unwrap_or(true),
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: None,
                updated_by: None,
            })),
            None => Ok(None),
        }
    }

    pub async fn create(&self, warehouse: CreateWarehouse) -> Result<Warehouse> {
        let result = sqlx::query!(
            "INSERT INTO warehouse.warehouses (warehouse_code, warehouse_name, city, state, country)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING warehouse_id, warehouse_code, warehouse_name, city, state, country, 
                      is_active, created_at, updated_at",
            warehouse.warehouse_code,
            warehouse.warehouse_name,
            warehouse.city,
            warehouse.state,
            warehouse.country.unwrap_or_else(|| "Indonesia".to_string())
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Warehouse {
            warehouse_id: result.warehouse_id,
            warehouse_code: result.warehouse_code,
            warehouse_name: result.warehouse_name,
            warehouse_type: None,
            address: None,
            city: result.city,
            state: result.state,
            postal_code: None,
            country: result.country,
            phone: None,
            email: None,
            manager_user_id: None,
            timezone: None,
            is_active: result.is_active.unwrap_or(true),
            created_at: result.created_at,
            updated_at: result.updated_at,
            created_by: None,
            updated_by: None,
        })
    }

    pub async fn update(&self, id: i32, warehouse: UpdateWarehouse) -> Result<Option<Warehouse>> {
        let result = sqlx::query!(
            "UPDATE warehouse.warehouses 
             SET warehouse_name = COALESCE($2, warehouse_name),
                 city = COALESCE($3, city),
                 state = COALESCE($4, state),
                 country = COALESCE($5, country),
                 updated_at = NOW()
             WHERE warehouse_id = $1 AND is_active = true
             RETURNING warehouse_id, warehouse_code, warehouse_name, city, state, country,
                      is_active, created_at, updated_at",
            id,
            warehouse.warehouse_name,
            warehouse.city,
            warehouse.state,
            warehouse.country
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Warehouse {
                warehouse_id: row.warehouse_id,
                warehouse_code: row.warehouse_code,
                warehouse_name: row.warehouse_name,
                warehouse_type: None,
                address: None,
                city: row.city,
                state: row.state,
                postal_code: None,
                country: row.country,
                phone: None,
                email: None,
                manager_user_id: None,
                timezone: None,
                is_active: row.is_active.unwrap_or(true),
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: None,
                updated_by: None,
            })),
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query!(
            "UPDATE warehouse.warehouses 
             SET is_active = false, updated_at = NOW()
             WHERE warehouse_id = $1 AND is_active = true",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn code_exists(&self, code: &str, exclude_id: Option<i32>) -> Result<bool> {
        let exists = match exclude_id {
            Some(id) => {
                sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM warehouse.warehouses 
                     WHERE warehouse_code = $1 AND warehouse_id != $2 AND is_active = true)",
                    code, id
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