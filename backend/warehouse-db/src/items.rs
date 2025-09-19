//! Item repository implementation

use sqlx::PgPool;
use warehouse_models::*;

#[derive(Clone)]
pub struct ItemRepository {
    pool: PgPool,
}

impl ItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // TODO: Implement item CRUD operations
    // This will be implemented in Phase 3
    pub async fn placeholder(&self) -> Result<()> {
        Ok(())
    }
}
