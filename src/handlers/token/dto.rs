use serde::Serialize;

/// Response model for token API endpoints
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub id: String, // MongoDB ObjectId as string
    pub network_id: u64,
    pub address: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<u8>,
    pub created_at: u64,
    pub updated_at: u64,
}
