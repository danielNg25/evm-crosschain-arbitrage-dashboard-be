use actix_web::{web, HttpResponse, Result};
use log::{error, info};
use mongodb::Database;

use crate::{
    errors::ApiError,
    handlers::{
        config::auth::ApiKey,
        network::{
            dto::{CreateNetworkRequest, UpdateFactoriesRequest, UpdateNetworkRequest},
            service::NetworkService,
        },
    },
};

/// GET /networks - Returns all networks
///
/// # Returns
/// JSON array of NetworkResponse objects containing network information
pub async fn get_networks_handler(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    info!("Handling GET /networks request");

    match NetworkService::get_networks_with_stats(&db).await {
        Ok(networks) => {
            info!("Successfully retrieved {} networks", networks.len());
            Ok(HttpResponse::Ok().json(networks))
        }
        Err(e) => {
            error!("Failed to retrieve networks: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve networks: {}",
                e
            )))
        }
    }
}

/// GET /networks/{chain_id} - Returns a specific network by chain_id
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing chain_id
///
/// # Returns
/// JSON object of NetworkResponse containing network information
pub async fn get_network_by_chain_id_handler(
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let chain_id = path.into_inner();
    info!("Handling GET /networks/{} request", chain_id);

    match NetworkService::get_network_by_chain_id(&db, chain_id).await {
        Ok(Some(network)) => {
            info!("Successfully retrieved network with chain_id: {}", chain_id);
            Ok(HttpResponse::Ok().json(network))
        }
        Ok(None) => {
            info!("Network with chain_id {} not found", chain_id);
            Err(ApiError::NotFound(format!(
                "Network with chain_id {} not found",
                chain_id
            )))
        }
        Err(e) => {
            error!("Failed to retrieve network {}: {}", chain_id, e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve network: {}",
                e
            )))
        }
    }
}

/// POST /networks - Creates a new network
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `body` - CreateNetworkRequest containing network data
///
/// # Returns
/// JSON object of NetworkResponse containing created network information
pub async fn create_network_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    body: web::Json<CreateNetworkRequest>,
) -> Result<HttpResponse, ApiError> {
    info!("Handling POST /networks request");

    match NetworkService::create_network(&db, body.into_inner()).await {
        Ok(network) => {
            info!(
                "Successfully created network with chain_id: {}",
                network.chain_id
            );
            Ok(HttpResponse::Created().json(network))
        }
        Err(e) => {
            error!("Failed to create network: {}", e);
            if e.to_string().contains("already exists") {
                Err(ApiError::BadRequest(e.to_string()))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to create network: {}",
                    e
                )))
            }
        }
    }
}

/// PUT /networks/{chain_id} - Updates an existing network
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `path` - Path parameters containing chain_id
/// * `body` - UpdateNetworkRequest containing fields to update
///
/// # Returns
/// JSON object of NetworkResponse containing updated network information
pub async fn update_network_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<u64>,
    body: web::Json<UpdateNetworkRequest>,
) -> Result<HttpResponse, ApiError> {
    let chain_id = path.into_inner();
    info!("Handling PUT /networks/{} request", chain_id);

    match NetworkService::update_network(&db, chain_id, body.into_inner()).await {
        Ok(network) => {
            info!("Successfully updated network with chain_id: {}", chain_id);
            Ok(HttpResponse::Ok().json(network))
        }
        Err(e) => {
            error!("Failed to update network {}: {}", chain_id, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Network with chain_id {} not found",
                    chain_id
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to update network: {}",
                    e
                )))
            }
        }
    }
}

/// POST /networks/{chain_id}/undelete - Undelete a network
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `path` - Path parameters containing chain_id
///
/// # Returns
/// JSON object of NetworkResponse containing undeleted network information
pub async fn undelete_network_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let chain_id = path.into_inner();
    info!("Handling POST /networks/{}/undelete request", chain_id);
    match NetworkService::undelete_network(&db, chain_id).await {
        Ok(network) => {
            info!("Successfully undeleted network with chain_id: {}", chain_id);
            Ok(HttpResponse::Ok().json(network))
        }
        Err(e) => {
            error!("Failed to undelete network {}: {}", chain_id, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Network with chain_id {} not found",
                    chain_id
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to undelete network: {}",
                    e
                )))
            }
        }
    }
}

/// PUT /networks/{chain_id}/factories - Updates both V2 factory fees and Aero factory addresses
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `path` - Path parameters containing chain_id
/// * `body` - UpdateFactoriesRequest containing both factory mappings
///
/// # Returns
/// JSON object of NetworkResponse containing updated network information
pub async fn update_factories_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<u64>,
    body: web::Json<UpdateFactoriesRequest>,
) -> Result<HttpResponse, ApiError> {
    let chain_id = path.into_inner();
    info!("Handling PUT /networks/{}/factories request", chain_id);

    match NetworkService::update_factories(&db, chain_id, body.into_inner()).await {
        Ok(network) => {
            info!(
                "Successfully updated factories for network with chain_id: {}",
                chain_id
            );
            Ok(HttpResponse::Ok().json(network))
        }
        Err(e) => {
            error!("Failed to update factories for network {}: {}", chain_id, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Network with chain_id {} not found",
                    chain_id
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to update factories: {}",
                    e
                )))
            }
        }
    }
}

/// DELETE /networks/{chain_id} - Deletes a network
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `path` - Path parameters containing chain_id
///
/// # Returns
/// 204 No Content on success
pub async fn delete_network_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let chain_id = path.into_inner();
    info!("Handling DELETE /networks/{} request", chain_id);

    match NetworkService::delete_network(&db, chain_id).await {
        Ok(_) => {
            info!("Successfully deleted network with chain_id: {}", chain_id);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(e) => {
            error!("Failed to delete network {}: {}", chain_id, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Network with chain_id {} not found",
                    chain_id
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to delete network: {}",
                    e
                )))
            }
        }
    }
}

/// DELETE /networks/{chain_id}/hard - Hard deletes a network (permanently removes from database)
/// Only works on networks that are already soft-deleted
/// Requires API key authentication via X-API-Key header
pub async fn hard_delete_network_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let chain_id = path.into_inner();
    info!("Handling DELETE /networks/{}/hard request", chain_id);

    match NetworkService::hard_delete_network(&db, chain_id).await {
        Ok(()) => {
            info!(
                "Successfully hard deleted network with chain_id: {}",
                chain_id
            );
            Ok(HttpResponse::NoContent().finish())
        }
        Err(e) => {
            error!("Failed to hard delete network {}: {}", chain_id, e);
            if e.to_string().contains("not found") || e.to_string().contains("not soft-deleted") {
                Err(ApiError::NotFound(format!(
                    "Network with chain_id {} not found or not soft-deleted",
                    chain_id
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to hard delete network: {}",
                    e
                )))
            }
        }
    }
}
