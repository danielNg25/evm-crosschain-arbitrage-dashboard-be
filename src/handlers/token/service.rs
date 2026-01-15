use alloy::primitives::Address;
use futures::TryStreamExt;
use log::debug;
use mongodb::bson::doc;
use mongodb::Database;

use crate::{
    database::models::utils::address_to_string, database::models::Token,
    handlers::token::dto::TokenResponse,
};

/// Service layer for token-related business logic
pub struct TokenService;

impl TokenService {
    /// Get all tokens
    ///
    /// # Arguments
    /// * `db` - Database reference
    ///
    /// # Returns
    /// * `Ok(Vec<TokenResponse>)` - List of tokens
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_all_tokens(db: &Database) -> anyhow::Result<Vec<TokenResponse>> {
        debug!("Fetching all tokens");

        let collection = db.collection::<Token>("tokens");
        let filter = doc! {};
        let mut cursor = collection.find(filter).await?;
        let mut tokens = Vec::new();

        while let Some(token) = cursor.try_next().await? {
            tokens.push(Self::map_to_response(token));
        }

        debug!("Retrieved {} tokens from database", tokens.len());
        Ok(tokens)
    }

    /// Get tokens by network ID
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `network_id` - The network ID to filter by
    ///
    /// # Returns
    /// * `Ok(Vec<TokenResponse>)` - List of tokens
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_tokens_by_network_id(
        db: &Database,
        network_id: u64,
    ) -> anyhow::Result<Vec<TokenResponse>> {
        debug!("Fetching tokens with network_id: {}", network_id);

        let collection = db.collection::<Token>("tokens");
        let filter = doc! { "network_id": network_id as i64 };
        let mut cursor = collection.find(filter).await?;
        let mut tokens = Vec::new();

        while let Some(token) = cursor.try_next().await? {
            tokens.push(Self::map_to_response(token));
        }

        debug!("Retrieved {} tokens from database", tokens.len());
        Ok(tokens)
    }

    /// Get a token by network ID and address
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `network_id` - The network ID
    /// * `address` - The token address
    ///
    /// # Returns
    /// * `Ok(Option<TokenResponse>)` - Token if found
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_token_by_address(
        db: &Database,
        network_id: u64,
        address: &Address,
    ) -> anyhow::Result<Option<TokenResponse>> {
        debug!(
            "Fetching token with network_id: {}, address: {}",
            network_id,
            address_to_string(address)
        );

        let collection = db.collection::<Token>("tokens");
        let addr_str = address_to_string(address);
        let filter = doc! {
            "network_id": network_id as i64,
            "address": &addr_str
        };
        let token = collection.find_one(filter).await?;

        if let Some(token) = token {
            Ok(Some(Self::map_to_response(token)))
        } else {
            Ok(None)
        }
    }

    /// Count tokens by network ID
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `network_id` - The network ID to count
    ///
    /// # Returns
    /// * `Ok(u64)` - Count of tokens
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn count_tokens_by_network_id(db: &Database, network_id: u64) -> anyhow::Result<u64> {
        debug!("Counting tokens with network_id: {}", network_id);

        let collection = db.collection::<Token>("tokens");
        let filter = doc! { "network_id": network_id as i64 };
        let count = collection.count_documents(filter).await?;

        Ok(count)
    }

    /// Map Token model to TokenResponse DTO
    ///
    /// # Arguments
    /// * `token` - Token model from database
    ///
    /// # Returns
    /// TokenResponse DTO
    fn map_to_response(token: Token) -> TokenResponse {
        let id = token
            .id
            .map(|oid| oid.to_hex())
            .unwrap_or_else(|| "unknown".to_string());

        TokenResponse {
            id,
            network_id: token.network_id,
            address: token.address,
            name: token.name,
            symbol: token.symbol,
            decimals: token.decimals,
            created_at: token.created_at,
            updated_at: token.updated_at,
        }
    }
}
