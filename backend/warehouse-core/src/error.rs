//! Centralized error handling for the warehouse system

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

/// Main application result type
pub type AppResult<T> = Result<T, AppError>;

/// Application-wide error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Already exists: {resource}")]
    AlreadyExists { resource: String },
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Forbidden: {reason}")]
    Forbidden { reason: String },
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    /// Convert validation errors to AppError
    pub fn validation<T: std::fmt::Display>(error: T) -> Self {
        Self::Validation(error.to_string())
    }
    
    /// Create not found error
    pub fn not_found(resource: &str) -> Self {
        Self::NotFound {
            resource: resource.to_string(),
        }
    }
    
    /// Create already exists error
    pub fn already_exists(resource: &str) -> Self {
        Self::AlreadyExists {
            resource: resource.to_string(),
        }
    }
    
    /// Create forbidden error
    pub fn forbidden(reason: &str) -> Self {
        Self::Forbidden {
            reason: reason.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message, error_code) = match &self {
            AppError::Database(_) => {
                error!("Database error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string(), "DATABASE_ERROR")
            }
            AppError::Validation(msg) => {
                (StatusCode::BAD_REQUEST, msg.clone(), "VALIDATION_ERROR")
            }
            AppError::NotFound { resource } => {
                (StatusCode::NOT_FOUND, format!("{} not found", resource), "NOT_FOUND")
            }
            AppError::AlreadyExists { resource } => {
                (StatusCode::CONFLICT, format!("{} already exists", resource), "ALREADY_EXISTS")
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized access".to_string(), "UNAUTHORIZED")
            }
            AppError::Forbidden { reason } => {
                (StatusCode::FORBIDDEN, reason.clone(), "FORBIDDEN")
            }
            AppError::Config(msg) => {
                error!("Configuration error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error".to_string(), "CONFIG_ERROR")
            }
            AppError::ExternalService { service, message } => {
                error!("External service {} error: {}", service, message);
                (StatusCode::BAD_GATEWAY, "External service error".to_string(), "EXTERNAL_SERVICE_ERROR")
            }
            AppError::Internal(_) => {
                error!("Internal error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string(), "INTERNAL_ERROR")
            }
        };

        let body = Json(json!({
            "success": false,
            "error": {
                "code": error_code,
                "message": message,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        }));

        (status, body).into_response()
    }
}
