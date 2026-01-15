use serde::{Deserialize, Serialize};

/// Response model for pool API endpoints
#[derive(Debug, Serialize)]
pub struct PoolResponse {
    pub id: String, // MongoDB ObjectId as string
    pub network_id: u64,
    pub address: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Request model for creating a new pool
#[derive(Debug, Deserialize)]
pub struct CreatePoolRequest {
    pub network_id: u64,
    pub address: String,
}

/// Request model for updating an existing pool
#[derive(Debug, Deserialize)]
pub struct UpdatePoolRequest {
    pub network_id: Option<u64>,
    pub address: Option<String>,
}
