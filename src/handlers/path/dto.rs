use serde::{Deserialize, Serialize};

/// Response model for path API endpoints
#[derive(Debug, Serialize)]
pub struct PathResponse {
    pub id: String, // MongoDB ObjectId as string
    pub paths: Vec<crate::bot::models::path::SingleChainPathsWithAnchorToken>,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted: bool,
}

/// Request model for creating a new path
#[derive(Debug, Deserialize)]
pub struct CreatePathRequest {
    pub paths: Vec<crate::bot::models::path::SingleChainPathsWithAnchorToken>,
}

/// Request model for updating an existing path
#[derive(Debug, Deserialize)]
pub struct UpdatePathRequest {
    pub paths: Option<Vec<crate::bot::models::path::SingleChainPathsWithAnchorToken>>,
}
