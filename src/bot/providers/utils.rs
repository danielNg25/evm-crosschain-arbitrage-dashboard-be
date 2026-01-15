use alloy::eips::BlockNumberOrTag;
use alloy::primitives::{Address, FixedBytes};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::rpc::types::{Filter, Log};
use alloy::transports::http::Http;
use alloy::transports::layers::FallbackLayer;
use anyhow::Result;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tower::ServiceBuilder;

pub async fn fetch_events(
    provider: Arc<DynProvider>,
    addresses: Vec<Address>,
    topics: Vec<FixedBytes<32>>,
    from_block: BlockNumberOrTag,
    to_block: BlockNumberOrTag,
) -> Result<Vec<Log>> {
    let filter = Filter::new()
        .from_block(from_block)
        .to_block(to_block)
        .address(addresses)
        .event_signature(topics);

    let events = provider.get_logs(&filter).await?;
    Ok(events)
}

pub fn create_provider(rpcs: Vec<String>) -> DynProvider {
    let rpc_len = rpcs.len();
    let fallback_layer =
        FallbackLayer::default().with_active_transport_count(NonZeroUsize::new(rpc_len).unwrap());

    // Define your list of transports to use
    let transports = rpcs
        .iter()
        .map(|url| Http::new(url.parse().unwrap()))
        .collect::<Vec<_>>();

    // Apply the FallbackLayer to the transports
    let transport = ServiceBuilder::new()
        .layer(fallback_layer)
        .service(transports);
    let client = RpcClient::builder().transport(transport, false);
    let provider = ProviderBuilder::new().connect_client(client.clone());
    provider.clone().erased()
}
