use alloy::primitives::Address;
use futures::TryStreamExt;
use log::debug;
use mongodb::bson::{self, doc, oid::ObjectId};
use mongodb::Database;
use std::collections::HashSet;
use std::str::FromStr;

use crate::{
    database::models::utils::address_to_string,
    database::models::Path,
    handlers::{
        path::dto::{CreatePathRequest, PathResponse, UpdatePathRequest},
        pool::service::PoolService,
    },
};

/// Service layer for path-related business logic
pub struct PathService;

impl PathService {
    /// Validate that an address string is a valid Ethereum address
    fn validate_address(address: &str) -> anyhow::Result<Address> {
        Address::from_str(address)
            .map_err(|e| anyhow::anyhow!("Invalid address format '{}': {}", address, e))
    }

    /// Validate all addresses in path data and path connectivity
    /// - Verifies addresses are not zero
    /// - Verifies first token_in equals anchor_token
    /// - Verifies each token_out equals the next token_in (path connectivity)
    fn validate_path_addresses(
        paths: &[crate::bot::models::path::SingleChainPathsWithAnchorToken],
    ) -> anyhow::Result<()> {
        use alloy::primitives::Address as AlloyAddress;

        for single_chain_path in paths {
            let anchor_token = single_chain_path.anchor_token;

            // Validate anchor_token is not zero address
            if anchor_token == AlloyAddress::ZERO {
                return Err(anyhow::anyhow!("Anchor token cannot be zero address"));
            }

            // Validate all pool paths
            for pool_path in &single_chain_path.paths {
                if pool_path.is_empty() {
                    return Err(anyhow::anyhow!("Pool path cannot be empty"));
                }

                // Verify first token_in equals anchor_token
                let first_direction = pool_path.first().unwrap();
                if first_direction.token_in != anchor_token {
                    return Err(anyhow::anyhow!(
                        "First token_in ({:?}) must equal anchor_token ({:?})",
                        first_direction.token_in,
                        anchor_token
                    ));
                }

                // Validate all addresses are not zero
                for pool_direction in pool_path {
                    if pool_direction.pool == AlloyAddress::ZERO {
                        return Err(anyhow::anyhow!("Pool address cannot be zero address"));
                    }
                    if pool_direction.token_in == AlloyAddress::ZERO {
                        return Err(anyhow::anyhow!("Token in address cannot be zero address"));
                    }
                    if pool_direction.token_out == AlloyAddress::ZERO {
                        return Err(anyhow::anyhow!("Token out address cannot be zero address"));
                    }
                }

                // Verify path connectivity: each token_out must equal next token_in
                let len = pool_path.len();
                if len >= 2 {
                    for i in 0..len - 1 {
                        if pool_path[i].token_out != pool_path[i + 1].token_in {
                            return Err(anyhow::anyhow!(
                                "Path connectivity error: token_out at index {} ({:?}) does not match token_in at index {} ({:?})",
                                i,
                                pool_path[i].token_out,
                                i + 1,
                                pool_path[i + 1].token_in
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
    /// Extract unique pools (network_id, address) from paths and ensure they exist
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `paths` - Vector of SingleChainPathsWithAnchorToken
    ///
    /// # Returns
    /// * `Ok(())` - All pools exist or were created
    /// * `Err(anyhow::Error)` - Error if database operation fails
    async fn ensure_pools_exist(
        db: &Database,
        paths: &[crate::bot::models::path::SingleChainPathsWithAnchorToken],
    ) -> anyhow::Result<()> {
        use alloy::primitives::Address;

        // Extract unique (network_id, pool_address) pairs
        let mut pool_set: HashSet<(u64, Address)> = HashSet::new();
        for single_chain_path in paths {
            for pool_path in &single_chain_path.paths {
                for pool_direction in pool_path {
                    pool_set.insert((single_chain_path.chain_id, pool_direction.pool));
                }
            }
        }

        debug!("Ensuring {} unique pools exist", pool_set.len());

        for (network_id, pool_address) in pool_set {
            let addr_str = address_to_string(&pool_address);
            PoolService::create_pool_if_not_exists(db, network_id, &addr_str).await?;
        }

        Ok(())
    }

    /// Get all paths
    ///
    /// # Arguments
    /// * `db` - Database reference
    ///
    /// # Returns
    /// * `Ok(Vec<PathResponse>)` - List of paths
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_all_paths(db: &Database) -> anyhow::Result<Vec<PathResponse>> {
        debug!("Fetching all paths");

        let collection = db.collection::<Path>("paths");
        // Return all paths (including deleted ones)
        let mut cursor = collection.find(doc! {}).await?;
        let mut paths = Vec::new();

        while let Some(path) = cursor.try_next().await? {
            paths.push(Self::map_to_response(path));
        }

        debug!("Retrieved {} paths from database", paths.len());
        Ok(paths)
    }

    /// Get a path by ID
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `id` - The ObjectId of the path to retrieve
    ///
    /// # Returns
    /// * `Ok(Option<PathResponse>)` - Path if found
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_path_by_id(
        db: &Database,
        id: &ObjectId,
    ) -> anyhow::Result<Option<PathResponse>> {
        debug!("Fetching path with id: {}", id);

        let collection = db.collection::<Path>("paths");
        // Return path even if deleted
        let filter = doc! { "_id": id };
        let path = collection.find_one(filter).await?;

        if let Some(path) = path {
            Ok(Some(Self::map_to_response(path)))
        } else {
            Ok(None)
        }
    }

    /// Get paths by anchor token
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `anchor_token` - The anchor token address
    ///
    /// # Returns
    /// * `Ok(Vec<PathResponse>)` - List of paths
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_paths_by_anchor_token(
        db: &Database,
        anchor_token: &str,
    ) -> anyhow::Result<Vec<PathResponse>> {
        debug!("Fetching paths with anchor_token: {}", anchor_token);

        let collection = db.collection::<Path>("paths");
        // Return all paths (including deleted ones)
        let filter = doc! {
            "paths": {
                "$elemMatch": {
                    "anchor_token": anchor_token
                }
            }
        };
        let mut cursor = collection.find(filter).await?;
        let mut paths = Vec::new();

        while let Some(path) = cursor.try_next().await? {
            paths.push(Self::map_to_response(path));
        }

        debug!("Retrieved {} paths from database", paths.len());
        Ok(paths)
    }

    /// Get paths by chain ID
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `chain_id` - The chain ID to filter by
    ///
    /// # Returns
    /// * `Ok(Vec<PathResponse>)` - List of paths
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_paths_by_chain_id(
        db: &Database,
        chain_id: u64,
    ) -> anyhow::Result<Vec<PathResponse>> {
        debug!("Fetching paths with chain_id: {}", chain_id);

        let collection = db.collection::<Path>("paths");
        // Return all paths (including deleted ones)
        let filter = doc! {
            "paths": {
                "$elemMatch": {
                    "chain_id": chain_id as i64
                }
            }
        };
        let mut cursor = collection.find(filter).await?;
        let mut paths = Vec::new();

        while let Some(path) = cursor.try_next().await? {
            paths.push(Self::map_to_response(path));
        }

        debug!("Retrieved {} paths from database", paths.len());
        Ok(paths)
    }

    /// Create a new path
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `request` - CreatePathRequest containing path data
    ///
    /// # Returns
    /// * `Ok(PathResponse)` - Created path
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn create_path(
        db: &Database,
        request: CreatePathRequest,
    ) -> anyhow::Result<PathResponse> {
        debug!("Creating new path");

        // Validate all addresses in paths before processing
        Self::validate_path_addresses(&request.paths)?;

        // Ensure all pools in the path exist
        Self::ensure_pools_exist(db, &request.paths).await?;

        let collection = db.collection::<Path>("paths");
        // Paths don't have a natural unique key, so always create new
        let path = Path::new(request.paths);
        let result = collection.insert_one(&path).await?;
        let id = result.inserted_id.as_object_id().unwrap();

        let filter = doc! { "_id": id };
        let created_path = collection.find_one(filter).await?.unwrap();

        debug!("Path created successfully with id: {}", id);
        Ok(Self::map_to_response(created_path))
    }

    /// Update an existing path
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `id` - The ObjectId of the path to update
    /// * `request` - UpdatePathRequest containing fields to update
    ///
    /// # Returns
    /// * `Ok(PathResponse)` - Updated path
    /// * `Err(anyhow::Error)` - Error if database operation fails or path not found
    pub async fn update_path(
        db: &Database,
        id: &ObjectId,
        request: UpdatePathRequest,
    ) -> anyhow::Result<PathResponse> {
        debug!("Updating path with id: {}", id);

        let collection = db.collection::<Path>("paths");
        // Only update non-deleted paths
        let filter = doc! {
            "_id": id,
            "$or": [
                { "deleted_at": null },
                { "deleted_at": { "$exists": false } }
            ]
        };

        // Check if path exists and is not deleted
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!("Path with id {} not found", id));
        }

        // Build update document
        let mut update_doc = mongodb::bson::Document::new();
        update_doc.insert("updated_at", chrono::Utc::now().timestamp() as i64);

        if let Some(paths) = &request.paths {
            // Validate path addresses and connectivity
            Self::validate_path_addresses(paths)?;

            // Ensure all pools in the updated path exist
            Self::ensure_pools_exist(db, paths).await?;
            update_doc.insert("paths", bson::to_bson(paths)?);
        }

        let update = doc! { "$set": update_doc };
        collection.update_one(filter.clone(), update).await?;

        // Get updated path
        let path = collection.find_one(filter).await?.unwrap();

        debug!("Path updated successfully: {}", id);
        Ok(Self::map_to_response(path))
    }

    /// Soft delete a path by ID (set deleted_at instead of removing)
    pub async fn delete_path(db: &Database, id: &ObjectId) -> anyhow::Result<()> {
        debug!("Soft deleting path with id: {}", id);

        let collection = db.collection::<Path>("paths");
        let filter = doc! {
            "_id": id,
            "$or": [
                { "deleted_at": null },
                { "deleted_at": { "$exists": false } }
            ]
        };

        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Path with id {} not found or already deleted",
                id
            ));
        }

        let update = doc! {
            "$set": {
                "deleted_at": chrono::Utc::now().timestamp() as i64,
                "updated_at": chrono::Utc::now().timestamp() as i64,
            }
        };

        collection.update_one(filter, update).await?;

        debug!("Path soft deleted successfully: {}", id);
        Ok(())
    }

    /// Hard delete a path (permanently remove from database)
    /// Only works on records that are already soft-deleted
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `id` - The ObjectId of the path to hard delete
    ///
    /// # Returns
    /// * `Ok(())` - Successfully deleted
    /// * `Err(anyhow::Error)` - Error if path not found or not soft-deleted
    pub async fn hard_delete_path(db: &Database, id: &ObjectId) -> anyhow::Result<()> {
        debug!("Hard deleting path with id: {}", id);

        let collection = db.collection::<Path>("paths");
        // Only hard delete if already soft-deleted
        let filter = doc! {
            "_id": id,
            "deleted_at": { "$ne": null, "$exists": true }
        };

        // Check if path exists and is soft-deleted
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Path with id {} not found or not soft-deleted",
                id
            ));
        }

        // Hard delete: actually remove from database
        collection.delete_one(filter).await?;

        debug!("Path hard deleted successfully: {}", id);
        Ok(())
    }

    /// Restore a soft-deleted path by setting deleted_at to null
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `id` - The ObjectId of the path to restore
    ///
    /// # Returns
    /// * `Ok(PathResponse)` - Restored path
    /// * `Err(anyhow::Error)` - Error if database operation fails or path not found
    pub async fn undelete_path(db: &Database, id: &ObjectId) -> anyhow::Result<PathResponse> {
        debug!("Restoring path with id: {}", id);

        let collection = db.collection::<Path>("paths");
        let filter = doc! { "_id": id };

        // Check if path exists
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!("Path with id {} not found", id));
        }

        let update = doc! {
            "$set": {
                "deleted_at": null,
                "updated_at": chrono::Utc::now().timestamp() as i64,
            }
        };

        collection.update_one(filter.clone(), update).await?;

        let restored_path = collection.find_one(filter).await?.unwrap();
        debug!("Path restored successfully: {}", id);
        Ok(Self::map_to_response(restored_path))
    }

    /// Map Path model to PathResponse DTO
    ///
    /// # Arguments
    /// * `path` - Path model from database
    ///
    /// # Returns
    /// PathResponse DTO
    fn map_to_response(path: Path) -> PathResponse {
        let id = path
            .id
            .map(|oid| oid.to_hex())
            .unwrap_or_else(|| "unknown".to_string());

        PathResponse {
            id,
            paths: path.paths,
            created_at: path.created_at,
            updated_at: path.updated_at,
            deleted: path.deleted_at.is_some(),
        }
    }
}
