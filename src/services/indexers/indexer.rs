#[async_trait::async_trait]
pub trait Indexer {
    async fn start(&mut self);
    async fn stop(&mut self);
    async fn is_running(&self) -> bool;
}
