use anyhow::Result;
use sqlx::PgPool;
use warehouse_models::*;
use crate::utils::*;

#[derive(Clone)]
pub struct ItemRepository {
    pool: PgPool,
}

impl ItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self, pagination: PaginationQuery) -> Result<PaginatedResponse<Item>> {
        let (page, limit) = validate_pagination(&pagination);
        let offset = calculate_offset(page, limit);

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM warehouse.items WHERE status = 'ACTIVE'"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        let rows = sqlx::query!(
            "SELECT * FROM warehouse.items WHERE status = 'ACTIVE' 
             ORDER BY item_name LIMIT $1 OFFSET $2",
            limit, offset
        )
        .fetch_all(&self.pool)
        .await?;

        let mut items = Vec::new();
        for row in rows {
            let item = Item {
                item_id: row.item_id,
                item_code: row.item_code,
                item_name: row.item_name,
                item_description: row.item_description,
                item_type: row.item_type,
                item_usage_type: row.item_usage_type,
                category: row.category,
                subcategory: row.subcategory,
                brand: row.brand,
                model: row.model,
                unit: row.unit,
                weight_kg: row.weight_kg,
                length_cm: row.length_cm,
                width_cm: row.width_cm,
                height_cm: row.height_cm,
                volume_cbm: row.volume_cbm,
                is_loanable: row.is_loanable.unwrap_or(false),
                requires_return: row.requires_return.unwrap_or(false),
                max_loan_duration_days: row.max_loan_duration_days,
                replacement_cost: row.replacement_cost,
                maintenance_required: row.maintenance_required.unwrap_or(false),
                calibration_required: row.calibration_required.unwrap_or(false),
                standard_cost: row.standard_cost,
                last_cost: row.last_cost,
                average_cost: row.average_cost,
                status: row.status,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: row.created_by,
                updated_by: row.updated_by,
            };
            items.push(item);
        }

        Ok(PaginatedResponse::new(items, total, page, limit))
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Item>> {
        let result = sqlx::query!(
            "SELECT * FROM warehouse.items WHERE item_id = $1 AND status = 'ACTIVE'",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Item {
                item_id: row.item_id,
                item_code: row.item_code,
                item_name: row.item_name,
                item_description: row.item_description,
                item_type: row.item_type,
                item_usage_type: row.item_usage_type,
                category: row.category,
                subcategory: row.subcategory,
                brand: row.brand,
                model: row.model,
                unit: row.unit,
                weight_kg: row.weight_kg,
                length_cm: row.length_cm,
                width_cm: row.width_cm,
                height_cm: row.height_cm,
                volume_cbm: row.volume_cbm,
                is_loanable: row.is_loanable.unwrap_or(false),
                requires_return: row.requires_return.unwrap_or(false),
                max_loan_duration_days: row.max_loan_duration_days,
                replacement_cost: row.replacement_cost,
                maintenance_required: row.maintenance_required.unwrap_or(false),
                calibration_required: row.calibration_required.unwrap_or(false),
                standard_cost: row.standard_cost,
                last_cost: row.last_cost,
                average_cost: row.average_cost,
                status: row.status,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: row.created_by,
                updated_by: row.updated_by,
            })),
            None => Ok(None),
        }
    }

    pub async fn create(&self, item: CreateItem) -> Result<Item> {
        let result = sqlx::query!(
            r#"
            INSERT INTO warehouse.items (
                item_code, item_name, item_description, item_type, item_usage_type,
                category, subcategory, brand, model, unit, is_loanable,
                maintenance_required, calibration_required, replacement_cost, created_by, updated_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#,
            item.item_code,
            item.item_name,
            item.item_description,
            item.item_type,
            item.item_usage_type,
            item.category,
            item.subcategory,
            item.brand,
            item.model,
            item.unit,
            item.is_loanable.unwrap_or(false),
            item.maintenance_required.unwrap_or(false),
            item.calibration_required.unwrap_or(false),
            item.replacement_cost,
            1i32, // created_by
            1i32  // updated_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Item {
            item_id: result.item_id,
            item_code: result.item_code,
            item_name: result.item_name,
            item_description: result.item_description,
            item_type: result.item_type,
            item_usage_type: result.item_usage_type,
            category: result.category,
            subcategory: result.subcategory,
            brand: result.brand,
            model: result.model,
            unit: result.unit,
            weight_kg: result.weight_kg,
            length_cm: result.length_cm,
            width_cm: result.width_cm,
            height_cm: result.height_cm,
            volume_cbm: result.volume_cbm,
            is_loanable: result.is_loanable.unwrap_or(false),
            requires_return: result.requires_return.unwrap_or(false),
            max_loan_duration_days: result.max_loan_duration_days,
            replacement_cost: result.replacement_cost,
            maintenance_required: result.maintenance_required.unwrap_or(false),
            calibration_required: result.calibration_required.unwrap_or(false),
            standard_cost: result.standard_cost,
            last_cost: result.last_cost,
            average_cost: result.average_cost,
            status: result.status,
            created_at: result.created_at,
            updated_at: result.updated_at,
            created_by: result.created_by,
            updated_by: result.updated_by,
        })
    }

    pub async fn code_exists(&self, code: &str, exclude_id: Option<i32>) -> Result<bool> {
        let exists = match exclude_id {
            Some(id) => {
                sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM warehouse.items 
                     WHERE item_code = $1 AND item_id != $2 AND status = 'ACTIVE')",
                    code, id
                )
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM warehouse.items 
                     WHERE item_code = $1 AND status = 'ACTIVE')",
                    code
                )
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(exists.unwrap_or(false))
    }
}
