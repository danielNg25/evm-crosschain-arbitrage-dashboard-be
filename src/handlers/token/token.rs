use actix_web::{web, HttpResponse};
use alloy::primitives::Address;
use log::{error, info};
use mongodb::Database;

use crate::{errors::ApiError, handlers::token::service::TokenService};

/// GET /tokens - Returns all tokens
///
/// # Returns
/// JSON array of TokenResponse objects containing token information
pub async fn get_tokens_handler(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    info!("Handling GET /tokens request");

    match TokenService::get_all_tokens(&db).await {
        Ok(tokens) => {
            info!("Successfully retrieved {} tokens", tokens.len());
            Ok(HttpResponse::Ok().json(tokens))
        }
        Err(e) => {
            error!("Failed to retrieve tokens: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve tokens: {}",
                e
            )))
        }
    }
}

/// GET /tokens/network/{network_id} - Returns tokens by network ID
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing network_id
///
/// # Returns
/// JSON array of TokenResponse objects
pub async fn get_tokens_by_network_id_handler(
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let network_id = path.into_inner();
    info!("Handling GET /tokens/network/{} request", network_id);

    match TokenService::get_tokens_by_network_id(&db, network_id).await {
        Ok(tokens) => {
            info!("Successfully retrieved {} tokens", tokens.len());
            Ok(HttpResponse::Ok().json(tokens))
        }
        Err(e) => {
            error!("Failed to retrieve tokens: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve tokens: {}",
                e
            )))
        }
    }
}

/// GET /tokens/network/{network_id}/address/{address} - Returns a specific token by network ID and address
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing network_id and address
///
/// # Returns
/// JSON object of TokenResponse containing token information
pub async fn get_token_by_address_handler(
    db: web::Data<Database>,
    path: web::Path<(u64, String)>,
) -> Result<HttpResponse, ApiError> {
    let (network_id, address_str) = path.into_inner();
    info!(
        "Handling GET /tokens/network/{}/address/{} request",
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

    match TokenService::get_token_by_address(&db, network_id, &address).await {
        Ok(Some(token)) => {
            info!("Successfully retrieved token");
            Ok(HttpResponse::Ok().json(token))
        }
        Ok(None) => {
            info!("Token not found");
            Err(ApiError::NotFound(format!(
                "Token with network_id {} and address {} not found",
                network_id, address_str
            )))
        }
        Err(e) => {
            error!("Failed to retrieve token: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve token: {}",
                e
            )))
        }
    }
}

/// GET /tokens/network/{network_id}/count - Returns count of tokens by network ID
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing network_id
///
/// # Returns
/// JSON object with count
pub async fn count_tokens_by_network_id_handler(
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let network_id = path.into_inner();
    info!("Handling GET /tokens/network/{}/count request", network_id);

    match TokenService::count_tokens_by_network_id(&db, network_id).await {
        Ok(count) => {
            info!("Successfully retrieved token count: {}", count);
            Ok(HttpResponse::Ok().json(serde_json::json!({ "count": count })))
        }
        Err(e) => {
            error!("Failed to retrieve token count: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve token count: {}",
                e
            )))
        }
    }
}
