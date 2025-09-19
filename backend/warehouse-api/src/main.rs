use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use validator::Validate;

use warehouse_core::{AppError, AppResult, AppState, Config};
use warehouse_db::Database;
use warehouse_models::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warehouse_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    info!("Starting warehouse system in {} mode", environment);

    // Database connection
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("../migrations").run(&pool).await?;
    
    let db = Database::new(pool);
    
    // Create config for AppState
    let config = Config {
        database_url: database_url.clone(),
        app_name: env::var("APP_NAME").unwrap_or_else(|_| "warehouse-api".to_string()),
        // Jika ada field lain di Config, tambahkan di sini. Pastikan sesuai dengan definisi Config di warehouse_core.
    };

    let app_state = AppState {
        db,
        config,
    };

    // Create router
    let app = create_app(app_state);

    // Start server
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("Server starting on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}


pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        // Warehouse routes
        .route("/api/warehouses", get(list_warehouses).post(create_warehouse))
        .route("/api/warehouses/:id", get(get_warehouse).put(update_warehouse).delete(delete_warehouse))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
        .with_state(state)
}

// ============================================================================
// HANDLERS
// ============================================================================

async fn root() -> &'static str {
    "Warehouse Management System API v1.0"
}

async fn health(State(state): State<AppState>) -> AppResult<Json<HealthStatus>> {
    let start_time = std::time::Instant::now();
    
    let database_health = match state.db.health_check().await {
        Ok(true) => ServiceHealth {
            status: "healthy".to_string(),
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            error: None,
        },
        Ok(false) => ServiceHealth {
            status: "unhealthy".to_string(),
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            error: Some("Database check returned false".to_string()),
        },
        Err(e) => ServiceHealth {
            status: "error".to_string(),
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            error: Some(e.to_string()),
        },
    };

    // Mock Redis health check for now
    let redis_health = ServiceHealth {
        status: "healthy".to_string(),
        response_time_ms: Some(1),
        error: None,
    };

    let health_status = HealthStatus {
        status: if database_health.status == "healthy" && redis_health.status == "healthy" {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        services: HealthServices {
            database: database_health,
            redis: redis_health,
        },
        uptime: "unknown".to_string(),
    };

    Ok(Json(health_status))
}

// ============================================================================
// WAREHOUSE HANDLERS
// ============================================================================

async fn list_warehouses(
    Query(pagination): Query<PaginationQuery>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<PaginatedResponse<Warehouse>>>> {
    let result = state.db.warehouses().list(pagination).await?;
    Ok(Json(ApiResponse::success(result)))
}

async fn create_warehouse(
    State(state): State<AppState>,
    Json(payload): Json<CreateWarehouse>,
) -> AppResult<Json<ApiResponse<Warehouse>>> {
    // Validate input
    payload.validate().map_err(|e| AppError::validation(e))?;

    // Check if code already exists
    if state.db.warehouses().code_exists(&payload.warehouse_code, None).await? {
        return Err(AppError::already_exists("warehouse with this code"));
    }

    let result = state.db.warehouses().create(payload).await?;
    Ok(Json(ApiResponse::success_with_message(
        result, 
        "Warehouse created successfully".to_string()
    )))
}

async fn get_warehouse(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Warehouse>>> {
    match state.db.warehouses().get_by_id(id).await? {
        Some(warehouse) => Ok(Json(ApiResponse::success(warehouse))),
        None => Err(AppError::not_found("warehouse")),
    }
}

async fn update_warehouse(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateWarehouse>,
) -> AppResult<Json<ApiResponse<Warehouse>>> {
    // Validate input
    payload.validate().map_err(|e| AppError::validation(e))?;

    match state.db.warehouses().update(id, payload).await? {
        Some(warehouse) => Ok(Json(ApiResponse::success_with_message(
            warehouse,
            "Warehouse updated successfully".to_string()
        ))),
        None => Err(AppError::not_found("warehouse")),
    }
}

async fn delete_warehouse(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<String>>> {
    if state.db.warehouses().delete(id).await? {
        Ok(Json(ApiResponse::success_with_message(
            "Warehouse deleted successfully".to_string(),
            "Operation completed".to_string()
        )))
    } else {
        Err(AppError::not_found("warehouse"))
    }
}