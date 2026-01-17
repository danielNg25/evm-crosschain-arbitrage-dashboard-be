use alloy::primitives::Address;
use futures::TryStreamExt;
use log::debug;
use mongodb::bson::doc;
use mongodb::Database;
use std::str::FromStr;

use crate::{
    database::models::Network,
    handlers::network::dto::{
        CreateNetworkRequest, NetworkResponse, UpdateFactoriesRequest, UpdateNetworkRequest,
    },
};

/// Service layer for network-related business logic
pub struct NetworkService;

impl NetworkService {
    /// Validate that an address string is a valid Ethereum address
    fn validate_address(address: &str) -> anyhow::Result<Address> {
        Address::from_str(address)
            .map_err(|e| anyhow::anyhow!("Invalid address format '{}': {}", address, e))
    }

    /// Validate all addresses in a network request
    fn validate_network_addresses(request: &CreateNetworkRequest) -> anyhow::Result<()> {
        // Validate wrap_native
        Self::validate_address(&request.wrap_native)?;

        // Validate multicall_address if provided
        if let Some(ref addr) = request.multicall_address {
            Self::validate_address(addr)?;
        }

        // Validate all addresses in v2_factory_to_fee keys
        if let Some(ref factory_to_fee) = request.v2_factory_to_fee {
            for factory_addr in factory_to_fee.keys() {
                Self::validate_address(factory_addr)?;
            }
        }

        // Validate all addresses in aero_factory_addresses
        if let Some(ref aero_addresses) = request.aero_factory_addresses {
            for addr in aero_addresses {
                Self::validate_address(addr)?;
            }
        }

        Ok(())
    }

    /// Validate addresses in an update request
    fn validate_update_addresses(request: &UpdateNetworkRequest) -> anyhow::Result<()> {
        if let Some(ref addr) = request.wrap_native {
            Self::validate_address(addr)?;
        }

        if let Some(ref addr) = request.multicall_address {
            Self::validate_address(addr)?;
        }

        if let Some(ref factory_to_fee) = request.v2_factory_to_fee {
            for factory_addr in factory_to_fee.keys() {
                Self::validate_address(factory_addr)?;
            }
        }

        if let Some(ref aero_addresses) = request.aero_factory_addresses {
            for addr in aero_addresses {
                Self::validate_address(addr)?;
            }
        }

        Ok(())
    }

    /// Validate addresses in factories update request
    fn validate_factories_addresses(request: &UpdateFactoriesRequest) -> anyhow::Result<()> {
        // Validate all addresses in v2_factory_to_fee keys
        for factory_addr in request.v2_factory_to_fee.keys() {
            Self::validate_address(factory_addr)?;
        }

        // Validate all addresses in aero_factory_addresses
        for addr in &request.aero_factory_addresses {
            Self::validate_address(addr)?;
        }

        Ok(())
    }
    /// Get all networks
    ///
    /// # Arguments
    /// * `db` - Database reference
    ///
    /// # Returns
    /// * `Ok(Vec<NetworkResponse>)` - List of networks
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_networks_with_stats(db: &Database) -> anyhow::Result<Vec<NetworkResponse>> {
        debug!("Fetching all networks (including deleted)");

        let collection = db.collection::<Network>("networks");
        // Return all networks, including deleted ones
        let filter = doc! {};
        let mut cursor = collection.find(filter).await?;
        let mut networks = Vec::new();

        while let Some(network) = cursor.try_next().await? {
            networks.push(Self::map_to_response(network));
        }

        debug!("Retrieved {} networks from database", networks.len());
        Ok(networks)
    }

    /// Map Network model to NetworkResponse DTO
    ///
    /// # Arguments
    /// * `network` - Network model from database
    ///
    /// # Returns
    /// NetworkResponse DTO
    fn map_to_response(network: Network) -> NetworkResponse {
        // Convert ObjectId to string
        let id = network
            .id
            .map(|oid| oid.to_hex())
            .unwrap_or_else(|| "unknown".to_string());

        // Determine if network is deleted
        let deleted = network.deleted_at.is_some();

        NetworkResponse {
            id,
            chain_id: network.chain_id,
            name: network.name,
            rpcs: network.rpcs,
            websocket_urls: network.websocket_urls,
            block_explorer: network.block_explorer,
            wrap_native: network.wrap_native,
            min_profit_usd: network.min_profit_usd,
            v2_factory_to_fee: network.v2_factory_to_fee,
            aero_factory_addresses: network.aero_factory_addresses,
            multicall_address: network.multicall_address,
            max_blocks_per_batch: network.max_blocks_per_batch,
            wait_time_fetch: network.wait_time_fetch,
            created_at: network.created_at,
            updated_at: network.updated_at,
            deleted,
        }
    }

    /// Get a single network by chain_id
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `chain_id` - The chain_id of the network to retrieve
    ///
    /// # Returns
    /// * `Ok(Option<NetworkResponse>)` - Network if found
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_network_by_chain_id(
        db: &Database,
        chain_id: u64,
    ) -> anyhow::Result<Option<NetworkResponse>> {
        debug!(
            "Fetching network with chain_id: {} (including deleted)",
            chain_id
        );

        let collection = db.collection::<Network>("networks");
        // Return network even if deleted
        let filter = doc! {
            "chain_id": chain_id as i64
        };
        let network = collection.find_one(filter).await?;

        if let Some(network) = network {
            Ok(Some(Self::map_to_response(network)))
        } else {
            Ok(None)
        }
    }

    /// Create a new network
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `request` - CreateNetworkRequest containing network data
    ///
    /// # Returns
    /// * `Ok(NetworkResponse)` - Created or restored network
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn create_network(
        db: &Database,
        request: CreateNetworkRequest,
    ) -> anyhow::Result<NetworkResponse> {
        debug!("Creating network with chain_id: {}", request.chain_id);

        // Validate all addresses before processing
        Self::validate_network_addresses(&request)?;

        let collection = db.collection::<Network>("networks");

        // Check if network exists (including soft-deleted)
        let filter = doc! { "chain_id": request.chain_id as i64 };
        let existing = collection.find_one(filter.clone()).await?;

        if let Some(mut existing_network) = existing {
            // Network exists, restore it and update with new data
            debug!(
                "Network with chain_id {} exists, restoring and updating",
                request.chain_id
            );

            let update = doc! {
                "$set": {
                    "name": &request.name,
                    "rpcs": &request.rpcs,
                    "websocket_urls": bson::to_bson(&request.websocket_urls)?,
                    "wrap_native": &request.wrap_native,
                    "min_profit_usd": request.min_profit_usd,
                    "block_explorer": bson::to_bson(&request.block_explorer)?,
                    "v2_factory_to_fee": bson::to_bson(&request.v2_factory_to_fee)?,
                    "aero_factory_addresses": bson::to_bson(&request.aero_factory_addresses)?,
                    "multicall_address": bson::to_bson(&request.multicall_address)?,
                    "max_blocks_per_batch": request.max_blocks_per_batch as i64,
                    "wait_time_fetch": request.wait_time_fetch as i64,
                    "updated_at": chrono::Utc::now().timestamp() as i64,
                    "deleted_at": null
                }
            };

            collection.update_one(filter.clone(), update).await?;
            let updated = collection.find_one(filter).await?.unwrap();
            return Ok(Self::map_to_response(updated));
        }

        // Create new network
        let network = Network::new(
            request.chain_id,
            request.name,
            request.rpcs,
            request.websocket_urls,
            request.wrap_native,
            request.min_profit_usd,
            request.block_explorer,
            request.v2_factory_to_fee,
            request.aero_factory_addresses,
            request.multicall_address,
            request.max_blocks_per_batch,
            request.wait_time_fetch,
        );

        collection.insert_one(&network).await?;

        debug!("Network created successfully: {}", request.chain_id);
        Ok(Self::map_to_response(network))
    }

    /// Update an existing network
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `chain_id` - The chain_id of the network to update
    /// * `request` - UpdateNetworkRequest containing fields to update
    ///
    /// # Returns
    /// * `Ok(NetworkResponse)` - Updated network
    /// * `Err(anyhow::Error)` - Error if database operation fails or network not found
    pub async fn update_network(
        db: &Database,
        chain_id: u64,
        request: UpdateNetworkRequest,
    ) -> anyhow::Result<NetworkResponse> {
        debug!("Updating network with chain_id: {}", chain_id);

        // Validate all addresses before processing
        Self::validate_update_addresses(&request)?;

        let collection = db.collection::<Network>("networks");
        let filter = doc! { "chain_id": chain_id as i64 };

        // Check if network exists
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Network with chain_id {} not found",
                chain_id
            ));
        }

        // Build update document
        let mut update_doc = mongodb::bson::Document::new();
        update_doc.insert("updated_at", chrono::Utc::now().timestamp() as i64);

        if let Some(name) = request.name {
            update_doc.insert("name", name);
        }
        if let Some(rpcs) = request.rpcs {
            update_doc.insert("rpcs", rpcs);
        }
        if let Some(websocket_urls) = request.websocket_urls {
            update_doc.insert("websocket_urls", websocket_urls);
        }
        if let Some(block_explorer) = request.block_explorer {
            update_doc.insert("block_explorer", block_explorer);
        }
        if let Some(wrap_native) = request.wrap_native {
            update_doc.insert("wrap_native", wrap_native);
        }
        if let Some(min_profit_usd) = request.min_profit_usd {
            update_doc.insert("min_profit_usd", min_profit_usd);
        }
        if let Some(v2_factory_to_fee) = request.v2_factory_to_fee {
            // Addresses already validated above
            let mut bson_map = mongodb::bson::Document::new();
            for (key, value) in v2_factory_to_fee {
                bson_map.insert(key, value as i64);
            }
            update_doc.insert("v2_factory_to_fee", bson_map);
        }
        if let Some(aero_factory_addresses) = request.aero_factory_addresses {
            update_doc.insert("aero_factory_addresses", aero_factory_addresses);
        }
        if let Some(multicall_address) = request.multicall_address {
            update_doc.insert("multicall_address", multicall_address);
        }
        if let Some(max_blocks_per_batch) = request.max_blocks_per_batch {
            update_doc.insert("max_blocks_per_batch", max_blocks_per_batch as i64);
        }
        if let Some(wait_time_fetch) = request.wait_time_fetch {
            update_doc.insert("wait_time_fetch", wait_time_fetch as i64);
        }

        let update = doc! { "$set": update_doc };
        collection.update_one(filter.clone(), update).await?;

        // Get updated network
        let network = collection.find_one(filter).await?.unwrap();

        debug!("Network updated successfully: {}", chain_id);
        Ok(Self::map_to_response(network))
    }

    /// Update both V2 factory to fee mapping and Aero factory addresses for a network
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `chain_id` - The chain_id of the network to update
    /// * `request` - UpdateFactoriesRequest containing both factory mappings
    ///
    /// # Returns
    /// * `Ok(NetworkResponse)` - Updated network
    /// * `Err(anyhow::Error)` - Error if database operation fails or network not found
    pub async fn update_factories(
        db: &Database,
        chain_id: u64,
        request: UpdateFactoriesRequest,
    ) -> anyhow::Result<NetworkResponse> {
        debug!(
            "Updating factories (V2 and Aero) for network with chain_id: {}",
            chain_id
        );

        // Validate all addresses before processing
        Self::validate_factories_addresses(&request)?;

        let collection = db.collection::<Network>("networks");
        let filter = doc! { "chain_id": chain_id as i64 };

        // Check if network exists
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Network with chain_id {} not found",
                chain_id
            ));
        }

        // Build V2 factory to fee BSON document
        let mut bson_map = mongodb::bson::Document::new();
        for (key, value) in request.v2_factory_to_fee {
            bson_map.insert(key, value as i64);
        }

        let update = doc! {
            "$set": {
                "v2_factory_to_fee": bson_map,
                "aero_factory_addresses": request.aero_factory_addresses,
                "updated_at": chrono::Utc::now().timestamp() as i64
            }
        };

        collection.update_one(filter.clone(), update).await?;

        // Get updated network
        let network = collection.find_one(filter).await?.unwrap();

        debug!(
            "Factories (V2 and Aero) updated successfully for network: {}",
            chain_id
        );
        Ok(Self::map_to_response(network))
    }

    /// Delete a network by chain_id
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `chain_id` - The chain_id of the network to delete
    ///
    /// # Returns
    /// * `Ok(())` - Network deleted successfully
    /// * `Err(anyhow::Error)` - Error if database operation fails or network not found
    pub async fn delete_network(db: &Database, chain_id: u64) -> anyhow::Result<()> {
        debug!("Soft deleting network with chain_id: {}", chain_id);

        let collection = db.collection::<Network>("networks");
        // Only soft-delete if not already deleted
        let filter = doc! {
            "chain_id": chain_id as i64,
            "$or": [
                { "deleted_at": null },
                { "deleted_at": { "$exists": false } }
            ]
        };

        // Check if network exists and is not already deleted
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Network with chain_id {} not found or already deleted",
                chain_id
            ));
        }

        // Soft delete: set deleted_at timestamp
        let update = doc! {
            "$set": {
                "deleted_at": chrono::Utc::now().timestamp() as i64,
                "updated_at": chrono::Utc::now().timestamp() as i64
            }
        };
        collection.update_one(filter, update).await?;

        debug!("Network soft deleted successfully: {}", chain_id);
        Ok(())
    }

    /// Hard delete a network (permanently remove from database)
    /// Only works on records that are already soft-deleted
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `chain_id` - The chain_id of the network to hard delete
    ///
    /// # Returns
    /// * `Ok(())` - Successfully deleted
    /// * `Err(anyhow::Error)` - Error if network not found or not soft-deleted
    pub async fn hard_delete_network(db: &Database, chain_id: u64) -> anyhow::Result<()> {
        debug!("Hard deleting network with chain_id: {}", chain_id);

        let collection = db.collection::<Network>("networks");
        // Only hard delete if already soft-deleted
        let filter = doc! {
            "chain_id": chain_id as i64,
            "deleted_at": { "$ne": null, "$exists": true }
        };

        // Check if network exists and is soft-deleted
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Network with chain_id {} not found or not soft-deleted",
                chain_id
            ));
        }

        // Hard delete: actually remove from database
        collection.delete_one(filter).await?;

        debug!("Network hard deleted successfully: {}", chain_id);
        Ok(())
    }

    /// Undelete (restore) a network by setting deleted_at to null
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `chain_id` - The chain_id of the network to restore
    ///
    /// # Returns
    /// * `Ok(NetworkResponse)` - Restored network
    /// * `Err(anyhow::Error)` - Error if database operation fails or network not found
    pub async fn undelete_network(db: &Database, chain_id: u64) -> anyhow::Result<NetworkResponse> {
        debug!("Undelete (restore) network with chain_id: {}", chain_id);

        let collection = db.collection::<Network>("networks");
        let filter = doc! { "chain_id": chain_id as i64 };

        // Check if network exists
        let existing = collection.find_one(filter.clone()).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Network with chain_id {} not found",
                chain_id
            ));
        }

        // Restore: set deleted_at to null
        let update = doc! {
            "$set": {
                "deleted_at": null,
                "updated_at": chrono::Utc::now().timestamp() as i64
            }
        };
        collection.update_one(filter.clone(), update).await?;

        // Get restored network
        let network = collection.find_one(filter).await?.unwrap();

        debug!("Network restored successfully: {}", chain_id);
        Ok(Self::map_to_response(network))
    }
}
