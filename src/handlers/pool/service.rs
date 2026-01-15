use alloy::eips::BlockId;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use futures::TryStreamExt;
use log::{debug, error, warn};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

use crate::{
    bot::providers::pool_fetcher::identify_and_fetch_pool,
    database::models::utils::address_to_string,
    database::models::{Network, Pool},
    handlers::pool::dto::{CreatePoolRequest, PoolResponse, UpdatePoolRequest},
};

/// Service layer for pool-related business logic
pub struct PoolService;

impl PoolService {
    /// Get all pools
    ///
    /// # Arguments
    /// * `db` - Database reference
    ///
    /// # Returns
    /// * `Ok(Vec<PoolResponse>)` - List of pools
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_all_pools(db: &Database) -> anyhow::Result<Vec<PoolResponse>> {
        debug!("Fetching all pools");

        let collection = db.collection::<Pool>("pools");
        let filter = doc! {};
        let mut cursor = collection.find(filter).await?;
        let mut pools = Vec::new();

        while let Some(pool) = cursor.try_next().await? {
            pools.push(Self::map_to_response(pool));
        }

        debug!("Retrieved {} pools from database", pools.len());
        Ok(pools)
    }

    /// Get pools by network ID
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `network_id` - The network ID to filter by
    ///
    /// # Returns
    /// * `Ok(Vec<PoolResponse>)` - List of pools
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_pools_by_network_id(
        db: &Database,
        network_id: u64,
    ) -> anyhow::Result<Vec<PoolResponse>> {
        debug!("Fetching pools with network_id: {}", network_id);

        let collection = db.collection::<Pool>("pools");
        let filter = doc! { "network_id": network_id as i64 };
        let mut cursor = collection.find(filter).await?;
        let mut pools = Vec::new();

        while let Some(pool) = cursor.try_next().await? {
            pools.push(Self::map_to_response(pool));
        }

        debug!("Retrieved {} pools from database", pools.len());
        Ok(pools)
    }

    /// Get a pool by network ID and address
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `network_id` - The network ID
    /// * `address` - The pool address
    ///
    /// # Returns
    /// * `Ok(Option<PoolResponse>)` - Pool if found
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_pool_by_address(
        db: &Database,
        network_id: u64,
        address: &Address,
    ) -> anyhow::Result<Option<PoolResponse>> {
        debug!(
            "Fetching pool with network_id: {}, address: {}",
            network_id,
            address_to_string(address)
        );

        let collection = db.collection::<Pool>("pools");
        let addr_str = address_to_string(address);
        let filter = doc! {
            "network_id": network_id as i64,
            "address": &addr_str
        };
        let pool = collection.find_one(filter).await?;

        if let Some(pool) = pool {
            Ok(Some(Self::map_to_response(pool)))
        } else {
            Ok(None)
        }
    }

    /// Count pools by network ID
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `network_id` - The network ID to count
    ///
    /// # Returns
    /// * `Ok(u64)` - Count of pools
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn count_pools_by_network_id(db: &Database, network_id: u64) -> anyhow::Result<u64> {
        debug!("Counting pools with network_id: {}", network_id);

        let collection = db.collection::<Pool>("pools");
        let filter = doc! { "network_id": network_id as i64 };
        let count = collection.count_documents(filter).await?;

        Ok(count)
    }

    /// Create a pool if it doesn't exist
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `network_id` - The network ID
    /// * `address` - The pool address
    ///
    /// # Returns
    /// * `Ok(())` - Pool exists or was created
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn create_pool_if_not_exists(
        db: &Database,
        network_id: u64,
        address: &str,
    ) -> anyhow::Result<()> {
        let collection = db.collection::<Pool>("pools");
        let filter = doc! {
            "network_id": network_id as i64,
            "address": address
        };

        // Check if pool exists
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            debug!(
                "Creating missing pool: network_id={}, address={}",
                network_id, address
            );
            let pool = Pool::new(network_id, address.to_string());
            collection.insert_one(pool).await?;
        }

        Ok(())
    }

    /// Create a new pool
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `request` - CreatePoolRequest containing pool data
    ///
    /// # Returns
    /// * `Ok(PoolResponse)` - Created pool
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn create_pool(
        db: &Database,
        request: CreatePoolRequest,
    ) -> anyhow::Result<PoolResponse> {
        debug!(
            "Creating new pool with network_id: {}, address: {}",
            request.network_id, request.address
        );

        let collection = db.collection::<Pool>("pools");
        let pool = Pool::new(request.network_id, request.address);
        let result = collection.insert_one(&pool).await?;
        let id = result.inserted_id.as_object_id().unwrap();

        let filter = doc! { "_id": id };
        let created_pool = collection.find_one(filter).await?.unwrap();

        debug!("Pool created successfully with id: {}", id);
        Ok(Self::map_to_response(created_pool))
    }

    /// Update an existing pool
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `id` - The ObjectId of the pool to update
    /// * `request` - UpdatePoolRequest containing fields to update
    ///
    /// # Returns
    /// * `Ok(PoolResponse)` - Updated pool
    /// * `Err(anyhow::Error)` - Error if database operation fails or pool not found
    pub async fn update_pool(
        db: &Database,
        id: &ObjectId,
        request: UpdatePoolRequest,
    ) -> anyhow::Result<PoolResponse> {
        debug!("Updating pool with id: {}", id);

        let collection = db.collection::<Pool>("pools");
        let filter = doc! { "_id": id };

        // Check if pool exists
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!("Pool with id {} not found", id));
        }

        // Build update document
        let mut update_doc = mongodb::bson::Document::new();
        update_doc.insert("updated_at", chrono::Utc::now().timestamp() as i64);

        if let Some(network_id) = request.network_id {
            update_doc.insert("network_id", network_id as i64);
        }
        if let Some(address) = request.address {
            update_doc.insert("address", address);
        }

        let update = doc! { "$set": update_doc };
        collection.update_one(filter.clone(), update).await?;

        // Get updated pool
        let pool = collection.find_one(filter).await?.unwrap();

        debug!("Pool updated successfully: {}", id);
        Ok(Self::map_to_response(pool))
    }

    /// Map Pool model to PoolResponse DTO
    ///
    /// # Arguments
    /// * `pool` - Pool model from database
    ///
    /// # Returns
    /// PoolResponse DTO
    fn map_to_response(pool: Pool) -> PoolResponse {
        let id = pool
            .id
            .map(|oid| oid.to_hex())
            .unwrap_or_else(|| "unknown".to_string());

        PoolResponse {
            id,
            network_id: pool.network_id,
            address: pool.address,
            created_at: pool.created_at,
            updated_at: pool.updated_at,
        }
    }
}
