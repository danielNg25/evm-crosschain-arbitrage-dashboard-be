// MongoDB modules
pub mod models;
pub mod mongodb;
// pub mod mongodb_logger;
pub mod repositories;
pub mod service;

// Re-export commonly used types
pub use mongodb::MongoDbClient;
pub use service::MongoDbService;

// Legacy sled Database (kept for backward compatibility if needed)
use anyhow::Result;
use log::{debug, info};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
