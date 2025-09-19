//! Warehouse Management System - Database Layer

use anyhow::Result;
use sqlx::PgPool;

pub mod repositories;
pub mod utils;

pub use repositories::*;
pub use utils::*;

/// Main database connection wrapper
#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Create new database instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get warehouse repository
    pub fn warehouses(&self) -> WarehouseRepository {
        WarehouseRepository::new(self.pool.clone())
    }

    /// Health check - test database connectivity
    pub async fn health_check(&self) -> Result<bool> {
        let row: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.0 == 1)
    }

    /// Get database version
    pub async fn version(&self) -> Result<String> {
        let row: (String,) = sqlx::query_as("SELECT version()")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.0)
    }
}
