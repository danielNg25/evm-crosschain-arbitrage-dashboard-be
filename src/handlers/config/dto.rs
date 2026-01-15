use serde::Serialize;

/// Response model for config API endpoints
#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub id: String, // MongoDB ObjectId as string
    pub max_amount_usd: f64,
    pub recheck_interval: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Request model for updating config
#[derive(Debug, serde::Deserialize)]
pub struct UpdateConfigRequest {
    pub max_amount_usd: Option<f64>,
    pub recheck_interval: Option<u64>,
}
