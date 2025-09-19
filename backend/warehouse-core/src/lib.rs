//! Warehouse Management System - Core Business Logic

pub mod config;
pub mod error;

pub use config::Config;
pub use error::{AppError, AppResult};

use warehouse_db::Database;

/// Main application state that holds all shared resources
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: Config,
}

impl AppState {
    pub fn new(db: Database, config: Config) -> Self {
        Self { db, config }
    }
}
