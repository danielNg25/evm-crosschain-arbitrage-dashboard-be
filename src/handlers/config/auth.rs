use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use log::warn;
use std::sync::Arc;

use crate::config::Config;

/// API Key extractor for authentication
pub struct ApiKey(pub String);

impl FromRequest for ApiKey {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Get the API key from app data
        let config = req
            .app_data::<web::Data<Arc<Config>>>()
            .map(|c| c.as_ref().clone());

        // Get the API key from the X-API-Key header
        let header_key = req
            .headers()
            .get("X-API-Key")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        match (config, header_key) {
            (Some(config), Some(header_key)) => {
                // Check if API key is configured
                if let Some(configured_key) = &config.server.api_key {
                    if configured_key == &header_key {
                        ready(Ok(ApiKey(header_key)))
                    } else {
                        warn!("Invalid API key provided");
                        ready(Err(actix_web::error::ErrorUnauthorized("Invalid API key")))
                    }
                } else {
                    // API key not configured, allow access (for development)
                    warn!("API key not configured, allowing access");
                    ready(Ok(ApiKey(header_key)))
                }
            }
            (Some(config), None) => {
                // API key required but not provided
                if config.server.api_key.is_some() {
                    warn!("API key required but not provided");
                    ready(Err(actix_web::error::ErrorUnauthorized("API key required")))
                } else {
                    // API key not configured, allow access (for development)
                    warn!("API key not configured, allowing access");
                    ready(Ok(ApiKey("".to_string())))
                }
            }
            _ => {
                warn!("Config not found in app data");
                ready(Err(actix_web::error::ErrorInternalServerError(
                    "Configuration error",
                )))
            }
        }
    }
}
