use actix_web::{web, HttpResponse};
use alloy::primitives::Address;
use log::{error, info};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;

use crate::{
    errors::ApiError,
    handlers::{
        config::auth::ApiKey,
        pool::{
            dto::{CreatePoolRequest, UpdatePoolRequest},
            service::PoolService,
        },
    },
};

/// GET /pools - Returns all pools
///
/// # Returns
/// JSON array of PoolResponse objects containing pool information
pub async fn get_pools_handler(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    info!("Handling GET /pools request");

    match PoolService::get_all_pools(&db).await {
        Ok(pools) => {
            info!("Successfully retrieved {} pools", pools.len());
            Ok(HttpResponse::Ok().json(pools))
        }
        Err(e) => {
            error!("Failed to retrieve pools: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve pools: {}",
                e
            )))
        }
    }
}

/// GET /pools/network/{network_id} - Returns pools by network ID
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing network_id
///
/// # Returns
/// JSON array of PoolResponse objects
pub async fn get_pools_by_network_id_handler(
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let network_id = path.into_inner();
    info!("Handling GET /pools/network/{} request", network_id);

    match PoolService::get_pools_by_network_id(&db, network_id).await {
        Ok(pools) => {
            info!("Successfully retrieved {} pools", pools.len());
            Ok(HttpResponse::Ok().json(pools))
        }
        Err(e) => {
            error!("Failed to retrieve pools: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve pools: {}",
                e
            )))
        }
    }
}

/// GET /pools/network/{network_id}/address/{address} - Returns a specific pool by network ID and address
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing network_id and address
///
/// # Returns
/// JSON object of PoolResponse containing pool information
pub async fn get_pool_by_address_handler(
    db: web::Data<Database>,
    path: web::Path<(u64, String)>,
) -> Result<HttpResponse, ApiError> {
    let (network_id, address_str) = path.into_inner();
    info!(
        "Handling GET /pools/network/{}/address/{} request",
        network_id, address_str
    );

    let address = match address_str.parse::<Address>() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Invalid address format: {}", e);
            return Err(ApiError::BadRequest(format!(
                "Invalid address format: {}",
                e
            )));
        }
    };

    match PoolService::get_pool_by_address(&db, network_id, &address).await {
        Ok(Some(pool)) => {
            info!("Successfully retrieved pool");
            Ok(HttpResponse::Ok().json(pool))
        }
        Ok(None) => {
            info!("Pool not found");
            Err(ApiError::NotFound(format!(
                "Pool with network_id {} and address {} not found",
                network_id, address_str
            )))
        }
        Err(e) => {
            error!("Failed to retrieve pool: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve pool: {}",
                e
            )))
        }
    }
}

/// GET /pools/network/{network_id}/count - Returns count of pools by network ID
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing network_id
///
/// # Returns
/// JSON object with count
pub async fn count_pools_by_network_id_handler(
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let network_id = path.into_inner();
    info!("Handling GET /pools/network/{}/count request", network_id);

    match PoolService::count_pools_by_network_id(&db, network_id).await {
        Ok(count) => {
            info!("Successfully retrieved pool count: {}", count);
            Ok(HttpResponse::Ok().json(serde_json::json!({ "count": count })))
        }
        Err(e) => {
            error!("Failed to retrieve pool count: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve pool count: {}",
                e
            )))
        }
    }
}

/// POST /pools - Creates a new pool
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `body` - CreatePoolRequest containing pool data
///
/// # Returns
/// JSON object of PoolResponse containing created pool information
pub async fn create_pool_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    body: web::Json<CreatePoolRequest>,
) -> Result<HttpResponse, ApiError> {
    info!("Handling POST /pools request");

    match PoolService::create_pool(&db, body.into_inner()).await {
        Ok(pool) => {
            info!("Successfully created pool with id: {}", pool.id);
            Ok(HttpResponse::Created().json(pool))
        }
        Err(e) => {
            error!("Failed to create pool: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to create pool: {}",
                e
            )))
        }
    }
}

/// PUT /pools/{id} - Updates an existing pool
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `path` - Path parameters containing id
/// * `body` - UpdatePoolRequest containing fields to update
///
/// # Returns
/// JSON object of PoolResponse containing updated pool information
pub async fn update_pool_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<UpdatePoolRequest>,
) -> Result<HttpResponse, ApiError> {
    let id_str = path.into_inner();
    info!("Handling PUT /pools/{} request", id_str);

    let id = match ObjectId::parse_str(&id_str) {
        Ok(id) => id,
        Err(e) => {
            error!("Invalid ObjectId format: {}", e);
            return Err(ApiError::BadRequest(format!("Invalid ID format: {}", e)));
        }
    };

    match PoolService::update_pool(&db, &id, body.into_inner()).await {
        Ok(pool) => {
            info!("Successfully updated pool with id: {}", id_str);
            Ok(HttpResponse::Ok().json(pool))
        }
        Err(e) => {
            error!("Failed to update pool {}: {}", id_str, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Pool with id {} not found",
                    id_str
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to update pool: {}",
                    e
                )))
            }
        }
    }
}

/// DELETE /pools/{id} - Soft deletes a pool (sets deleted_at)
/// Requires API key authentication via X-API-Key header
pub async fn delete_pool_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id_str = path.into_inner();
    info!("Handling DELETE /pools/{} request", id_str);

    let id = match ObjectId::parse_str(&id_str) {
        Ok(id) => id,
        Err(e) => {
            error!("Invalid ObjectId format: {}", e);
            return Err(ApiError::BadRequest(format!("Invalid ID format: {}", e)));
        }
    };

    match PoolService::delete_pool(&db, &id).await {
        Ok(()) => {
            info!("Successfully soft deleted pool with id: {}", id_str);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(e) => {
            error!("Failed to delete pool {}: {}", id_str, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Pool with id {} not found",
                    id_str
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to delete pool: {}",
                    e
                )))
            }
        }
    }
}
