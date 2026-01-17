use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Response models for API endpoints
#[derive(Debug, Serialize)]
pub struct NetworkResponse {
    pub id: String, // MongoDB ObjectId as string
    pub chain_id: u64,
    pub name: String,
    pub rpcs: Vec<String>,
    pub websocket_urls: Option<Vec<String>>,
    pub block_explorer: Option<String>,
    pub wrap_native: String,
    pub min_profit_usd: f64,
    pub v2_factory_to_fee: Option<HashMap<String, u64>>,
    pub aero_factory_addresses: Option<Vec<String>>,
    pub multicall_address: Option<String>,
    pub max_blocks_per_batch: u64,
    pub wait_time_fetch: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted: bool,
}

// Request model for creating a new network
#[derive(Debug, Deserialize)]
pub struct CreateNetworkRequest {
    pub chain_id: u64,
    pub name: String,
    pub rpcs: Vec<String>,
    pub websocket_urls: Option<Vec<String>>,
    pub block_explorer: Option<String>,
    pub wrap_native: String,
    pub min_profit_usd: f64,
    pub v2_factory_to_fee: Option<HashMap<String, u64>>,
    pub aero_factory_addresses: Option<Vec<String>>,
    pub multicall_address: Option<String>,
    pub max_blocks_per_batch: u64,
    pub wait_time_fetch: u64,
}

// Request model for updating an existing network
#[derive(Debug, Deserialize)]
pub struct UpdateNetworkRequest {
    pub name: Option<String>,
    pub rpcs: Option<Vec<String>>,
    pub websocket_urls: Option<Vec<String>>,
    pub block_explorer: Option<String>,
    pub wrap_native: Option<String>,
    pub min_profit_usd: Option<f64>,
    pub v2_factory_to_fee: Option<HashMap<String, u64>>,
    pub aero_factory_addresses: Option<Vec<String>>,
    pub multicall_address: Option<String>,
    pub max_blocks_per_batch: Option<u64>,
    pub wait_time_fetch: Option<u64>,
}

// Request model for updating both V2 and Aero factories together
#[derive(Debug, Deserialize)]
pub struct UpdateFactoriesRequest {
    pub v2_factory_to_fee: HashMap<String, u64>,
    pub aero_factory_addresses: Vec<String>,
}
