use log::info;
use std::time::Duration;

use crate::services::Indexer;

pub struct SimpleIndexer {
    running: bool,
}

impl SimpleIndexer {
    pub fn new() -> Self {
        Self { running: false }
    }
}

#[async_trait::async_trait]
impl Indexer for SimpleIndexer {
    async fn start(&mut self) {
        self.running = true;

        while self.running {
            info!("Simple indexer is still running");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn stop(&mut self) {
        self.running = false;
    }

    async fn is_running(&self) -> bool {
        self.running
    }
}
