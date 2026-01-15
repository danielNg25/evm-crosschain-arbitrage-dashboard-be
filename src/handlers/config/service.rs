use log::debug;
use mongodb::bson::doc;
use mongodb::Database;

use crate::{database::models::Config, handlers::config::dto::ConfigResponse};

/// Service layer for config-related business logic
pub struct ConfigService;

impl ConfigService {
    /// Get the config (singleton pattern)
    ///
    /// # Arguments
    /// * `db` - Database reference
    ///
    /// # Returns
    /// * `Ok(Option<ConfigResponse>)` - Config if exists
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn get_config(db: &Database) -> anyhow::Result<Option<ConfigResponse>> {
        debug!("Fetching config");

        let collection = db.collection::<Config>("configs");
        let config = collection.find_one(doc! {}).await?;

        if let Some(config) = config {
            Ok(Some(Self::map_to_response(config)))
        } else {
            Ok(None)
        }
    }

    /// Map Config model to ConfigResponse DTO
    ///
    /// # Arguments
    /// * `config` - Config model from database
    ///
    /// # Returns
    /// ConfigResponse DTO
    fn map_to_response(config: Config) -> ConfigResponse {
        let id = config
            .id
            .map(|oid| oid.to_hex())
            .unwrap_or_else(|| "unknown".to_string());

        ConfigResponse {
            id,
            max_amount_usd: config.max_amount_usd,
            recheck_interval: config.recheck_interval,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }

    /// Update config
    ///
    /// # Arguments
    /// * `db` - Database reference
    /// * `max_amount_usd` - Optional new max_amount_usd value
    /// * `recheck_interval` - Optional new recheck_interval value
    ///
    /// # Returns
    /// * `Ok(ConfigResponse)` - Updated config
    /// * `Err(anyhow::Error)` - Error if database operation fails
    pub async fn update_config(
        db: &Database,
        max_amount_usd: Option<f64>,
        recheck_interval: Option<u64>,
    ) -> anyhow::Result<ConfigResponse> {
        debug!("Updating config");

        let collection = db.collection::<Config>("configs");
        let filter = doc! {};

        let mut update_doc = mongodb::bson::Document::new();
        update_doc.insert("updated_at", chrono::Utc::now().timestamp() as i64);

        if let Some(max_amount_usd) = max_amount_usd {
            update_doc.insert("max_amount_usd", max_amount_usd);
        }

        if let Some(recheck_interval) = recheck_interval {
            update_doc.insert("recheck_interval", recheck_interval as i64);
        }

        let update = doc! { "$set": update_doc };
        let result = collection.update_one(filter.clone(), update).await?;

        if result.matched_count == 0 {
            // Create new config if it doesn't exist
            let default_max_amount_usd = max_amount_usd.unwrap_or(1000.0);
            let default_recheck_interval = recheck_interval.unwrap_or(60);
            let new_config = Config::new(default_max_amount_usd, default_recheck_interval);
            collection.insert_one(new_config.clone()).await?;
            Ok(Self::map_to_response(new_config))
        } else {
            // Get updated config
            let config = collection.find_one(filter).await?.unwrap();
            Ok(Self::map_to_response(config))
        }
    }
}
