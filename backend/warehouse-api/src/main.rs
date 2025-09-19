use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use warehouse_core::{AppError, AppResult, AppState, Config};
use warehouse_db::Database;
use warehouse_models::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warehouse_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    config.validate()?;

    info!("Starting warehouse system in {} mode", config.server.environment);

    let pool = PgPool::connect(&config.database.url).await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    
    let db = Database::new(pool);
    let app_state = AppState::new(db, config.clone());

    let app = create_app(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("Server starting on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/warehouses", get(list_warehouses).post(create_warehouse))
        .route("/api/warehouses/:id", get(get_warehouse).put(update_warehouse).delete(delete_warehouse))
        .route("/api/items", get(list_items).post(create_item))
        .route("/api/items/:id", get(get_item))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
        .with_state(state)
}

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

async fn list_warehouses(
    Query(pagination): Query<PaginationQuery>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<PaginatedResponse<Warehouse>>>> {
    let result = state.db.warehouses().list(pagination).await?;
    Ok(Json(ApiResponse::success(result)))
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

// Items handlers
async fn list_items(
    Query(pagination): Query<PaginationQuery>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<PaginatedResponse<Item>>>> {
    let result = state.db.items().list(pagination).await?;
    Ok(Json(ApiResponse::success(result)))
}

async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> AppResult<Json<ApiResponse<Item>>> {
    payload.validate().map_err(|e| AppError::validation(e))?;

    if state.db.items().code_exists(&payload.item_code, None).await? {
        return Err(AppError::already_exists("item with this code"));
    }

    let result = state.db.items().create(payload).await?;
    Ok(Json(ApiResponse::success_with_message(
        result, 
        "Item created successfully".to_string()
    )))
}

async fn get_item(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Item>>> {
    match state.db.items().get_by_id(id).await? {
        Some(item) => Ok(Json(ApiResponse::success(item))),
        None => Err(AppError::not_found("item")),
    }
}
