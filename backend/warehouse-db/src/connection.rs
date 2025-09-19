//! Database connection management

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use warehouse_models::*;

pub struct DatabaseManager;

impl DatabaseManager {
    /// Create a new database connection pool with optimized settings
    pub async fn connect(database_url: &str) -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(3600))
            .test_before_acquire(true)
            .connect(database_url)
            .await
            .map_err(|e| {
                tracing::error!("Failed to connect to database: {}", e);
                WarehouseError::Database(e)
            })?;

        tracing::info!("Database connection pool created successfully");
        Ok(pool)
    }

    /// Run database migrations
    pub async fn run_migrations(pool: &PgPool) -> Result<()> {
        sqlx::migrate!("../migrations")
            .run(pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to run migrations: {}", e);
                WarehouseError::Database(e)
            })?;

        tracing::info!("Database migrations completed successfully");
        Ok(())
    }

    /// Verify database schema and connectivity
    pub async fn verify_database(pool: &PgPool) -> Result<()> {
        // Check if main tables exist
        let tables = vec!["warehouses", "items", "projects"];
        
        for table in tables {
            let exists = sqlx::query_scalar!(
                "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = $1)",
                table
            )
            .fetch_one(pool)
            .await?;

            if !exists.unwrap_or(false) {
                return Err(WarehouseError::Internal(
                    format!("Required table '{}' does not exist", table)
                ));
            }
        }

        tracing::info!("Database schema verification completed");
        Ok(())
    }
}
