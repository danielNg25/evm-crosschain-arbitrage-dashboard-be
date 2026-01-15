use crate::bot::models::pool::base::{PoolInterface, PoolType, Topic};
use alloy::primitives::Address;
use alloy::providers::DynProvider;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct PoolRegistry {
    provider: Arc<DynProvider>,
    by_address: Arc<RwLock<HashMap<Address, Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>>>>,
    by_type: Arc<RwLock<HashMap<PoolType, Vec<Address>>>>,
    last_processed_block: Arc<RwLock<u64>>,
    topics: Arc<RwLock<Vec<Topic>>>,
    profitable_topics: Arc<RwLock<HashSet<Topic>>>,
    pub factory_to_fee: Arc<RwLock<HashMap<String, u64>>>,
    pub aero_factory_addresses: Arc<RwLock<Vec<Address>>>,
    network_id: u64,
}

impl PoolRegistry {
    pub fn new(provider: DynProvider, network_id: u64) -> Self {
        Self {
            provider: Arc::new(provider),
            by_address: Arc::new(RwLock::new(HashMap::new())),
            by_type: Arc::new(RwLock::new(HashMap::new())),
            last_processed_block: Arc::new(RwLock::new(0)),
            topics: Arc::new(RwLock::new(Vec::new())),
            profitable_topics: Arc::new(RwLock::new(HashSet::new())),
            factory_to_fee: Arc::new(RwLock::new(HashMap::new())),
            aero_factory_addresses: Arc::new(RwLock::new(Vec::new())),
            network_id,
        }
    }

    pub async fn set_provider(&mut self, provider: DynProvider) {
        self.provider = Arc::new(provider);
    }

    pub async fn set_factory_to_fee(&self, factory_to_fee: HashMap<String, u64>) {
        *self.factory_to_fee.write().await = factory_to_fee.clone();
    }

    pub async fn set_aero_factory_addresses(&self, aero_factory_addresses: Vec<Address>) {
        *self.aero_factory_addresses.write().await = aero_factory_addresses.clone();
    }

    /// Set network ID for this registry
    pub fn set_network_id(&mut self, network_id: u64) {
        self.network_id = network_id;
    }

    /// Get network ID
    pub fn get_network_id(&self) -> u64 {
        self.network_id
    }

    /// Get total pool count
    pub async fn pool_count(&self) -> usize {
        self.by_address.read().await.len()
    }

    pub async fn add_pool(&self, pool: Box<dyn PoolInterface + Send + Sync>) {
        let address = pool.address();
        let pool_type = pool.pool_type();

        // Add to address map
        let mut address_map = self.by_address.write().await;
        address_map.insert(address, Arc::new(RwLock::new(pool)));

        // Add to type map
        let mut type_map = self.by_type.write().await;
        type_map
            .entry(pool_type)
            .or_insert_with(Vec::new)
            .push(address);
    }

    pub async fn exists_pool(&self, address: &Address) -> bool {
        self.by_address.read().await.contains_key(address)
    }

    pub async fn get_pool(
        &self,
        address: &Address,
    ) -> Option<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let pools = self.by_address.read().await;
        pools.get(address).map(Arc::clone)
    }

    pub async fn remove_pool(
        &self,
        address: Address,
    ) -> Option<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        // Remove from address map
        let mut address_map = self.by_address.write().await;
        let pool = address_map.remove(&address)?;
        let pool_type = pool.read().await.pool_type();

        // Remove from type map
        let mut type_map = self.by_type.write().await;
        if let Some(addresses) = type_map.get_mut(&pool_type) {
            addresses.retain(|&a| a != address);
            if addresses.is_empty() {
                type_map.remove(&pool_type);
            }
        }

        Some(pool)
    }

    pub async fn get_all_pools(&self) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let pools = self.by_address.read().await;
        pools.values().map(Arc::clone).collect()
    }

    pub async fn get_pools_by_type(
        &self,
        pool_type: PoolType,
    ) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let type_map = self.by_type.read().await;
        let address_map = self.by_address.read().await;

        type_map
            .get(&pool_type)
            .map(|addresses| {
                addresses
                    .iter()
                    .filter_map(|addr| address_map.get(addr).map(Arc::clone))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn get_v2_pools(&self) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let type_map = self.by_type.read().await;
        let address_map = self.by_address.read().await;

        type_map
            .get(&PoolType::UniswapV2)
            .map(|addresses| {
                addresses
                    .iter()
                    .filter_map(|addr| address_map.get(addr).map(Arc::clone))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn get_v3_pools(&self) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let type_map = self.by_type.read().await;
        let address_map = self.by_address.read().await;

        type_map
            .get(&PoolType::UniswapV3)
            .map(|addresses| {
                addresses
                    .iter()
                    .filter_map(|addr| address_map.get(addr).map(Arc::clone))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn get_addresses_by_type(&self, pool_type: PoolType) -> Vec<Address> {
        let type_map = self.by_type.read().await;
        type_map
            .get(&pool_type)
            .map(|addresses| addresses.clone())
            .unwrap_or_default()
    }

    pub async fn get_v2_addresses(&self) -> Vec<Address> {
        self.get_addresses_by_type(PoolType::UniswapV2).await
    }

    pub async fn get_v3_addresses(&self) -> Vec<Address> {
        self.get_addresses_by_type(PoolType::UniswapV3).await
    }

    pub async fn get_all_addresses(&self) -> Vec<Address> {
        self.by_address.read().await.keys().cloned().collect()
    }

    pub async fn get_factory_to_fee(&self) -> HashMap<String, u64> {
        self.factory_to_fee.read().await.clone()
    }

    pub async fn get_aero_factory_addresses(&self) -> Vec<Address> {
        self.aero_factory_addresses.read().await.clone()
    }

    pub async fn log_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Pool Registry Summary:\n");
        summary.push_str("--------------------------------\n");

        let pools = self.by_address.read().await;
        for (_, pool) in &*pools {
            summary.push_str(&format!("Pool: {}\n", pool.read().await.log_summary()));
        }
        summary
    }

    // Get the last processed block
    pub async fn get_last_processed_block(&self) -> u64 {
        *self.last_processed_block.read().await
    }

    // Set the last processed block
    pub async fn set_last_processed_block(&self, block_number: u64) {
        let mut block = self.last_processed_block.write().await;
        *block = block_number;
    }

    pub async fn add_topics(&self, topics: Vec<Topic>) {
        let mut topics_lock = self.topics.write().await;
        topics_lock.extend(topics);
    }

    pub async fn add_profitable_topics(&self, topics: Vec<Topic>) {
        let mut topics_lock = self.profitable_topics.write().await;
        topics_lock.extend(topics);
    }

    pub async fn get_topics(&self) -> Vec<Topic> {
        self.topics.read().await.clone()
    }

    pub async fn get_profitable_topics(&self) -> HashSet<Topic> {
        self.profitable_topics.read().await.clone()
    }
}

impl Clone for PoolRegistry {
    fn clone(&self) -> Self {
        Self {
            provider: Arc::clone(&self.provider),
            by_address: Arc::clone(&self.by_address),
            by_type: Arc::clone(&self.by_type),
            last_processed_block: Arc::clone(&self.last_processed_block),
            topics: Arc::clone(&self.topics),
            profitable_topics: Arc::clone(&self.profitable_topics),
            factory_to_fee: Arc::clone(&self.factory_to_fee),
            aero_factory_addresses: Arc::clone(&self.aero_factory_addresses),
            network_id: self.network_id.clone(),
        }
    }
}
