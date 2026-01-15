use actix_web::{web, HttpResponse};
use log::{error, info};
use mongodb::Database;

use crate::{
    errors::ApiError,
    handlers::config::{auth::ApiKey, service::ConfigService},
};

/// GET /config - Returns the current config
///
/// # Returns
/// JSON object of ConfigResponse containing config information
pub async fn get_config_handler(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    info!("Handling GET /config request");

    match ConfigService::get_config(&db).await {
        Ok(Some(config)) => {
            info!("Successfully retrieved config");
            Ok(HttpResponse::Ok().json(config))
        }
        Ok(None) => {
            info!("Config not found");
            Err(ApiError::NotFound("Config not found".to_string()))
        }
        Err(e) => {
            error!("Failed to retrieve config: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve config: {}",
                e
            )))
        }
    }
}

/// PUT /config - Updates the config
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `body` - UpdateConfigRequest containing fields to update
///
/// # Returns
/// JSON object of ConfigResponse containing updated config information
pub async fn update_config_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    body: web::Json<crate::handlers::config::dto::UpdateConfigRequest>,
) -> Result<HttpResponse, ApiError> {
    info!("Handling PUT /config request");

    match ConfigService::update_config(&db, body.max_amount_usd, body.recheck_interval).await {
        Ok(config) => {
            info!("Successfully updated config");
            Ok(HttpResponse::Ok().json(config))
        }
        Err(e) => {
            error!("Failed to update config: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to update config: {}",
                e
            )))
        }
    }
}
