// warehouse-models/src/warehouse.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateWarehouseRequest {
    #[validate(length(min = 1, max = 50))]
    pub warehouse_code: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    pub warehouse_name: Option<String>,
    
    pub warehouse_type: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub manager_user_id: Option<i32>,
    pub is_active: Option<bool>,
}