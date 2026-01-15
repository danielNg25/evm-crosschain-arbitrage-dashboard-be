use actix_web::{web, HttpResponse};
use log::{error, info};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;

use crate::{
    errors::ApiError,
    handlers::{
        config::auth::ApiKey,
        path::{
            dto::{CreatePathRequest, UpdatePathRequest},
            service::PathService,
        },
    },
};

/// GET /paths - Returns all paths
///
/// # Returns
/// JSON array of PathResponse objects containing path information
pub async fn get_paths_handler(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    info!("Handling GET /paths request");

    match PathService::get_all_paths(&db).await {
        Ok(paths) => {
            info!("Successfully retrieved {} paths", paths.len());
            Ok(HttpResponse::Ok().json(paths))
        }
        Err(e) => {
            error!("Failed to retrieve paths: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve paths: {}",
                e
            )))
        }
    }
}

/// GET /paths/{id} - Returns a specific path by ID
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing id
///
/// # Returns
/// JSON object of PathResponse containing path information
pub async fn get_path_by_id_handler(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id_str = path.into_inner();
    info!("Handling GET /paths/{} request", id_str);

    let id = match ObjectId::parse_str(&id_str) {
        Ok(id) => id,
        Err(e) => {
            error!("Invalid ObjectId format: {}", e);
            return Err(ApiError::BadRequest(format!("Invalid ID format: {}", e)));
        }
    };

    match PathService::get_path_by_id(&db, &id).await {
        Ok(Some(path)) => {
            info!("Successfully retrieved path with id: {}", id_str);
            Ok(HttpResponse::Ok().json(path))
        }
        Ok(None) => {
            info!("Path with id {} not found", id_str);
            Err(ApiError::NotFound(format!(
                "Path with id {} not found",
                id_str
            )))
        }
        Err(e) => {
            error!("Failed to retrieve path {}: {}", id_str, e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve path: {}",
                e
            )))
        }
    }
}

/// GET /paths/anchor-token/{anchor_token} - Returns paths by anchor token
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing anchor_token
///
/// # Returns
/// JSON array of PathResponse objects
pub async fn get_paths_by_anchor_token_handler(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let anchor_token = path.into_inner();
    info!("Handling GET /paths/anchor-token/{} request", anchor_token);

    match PathService::get_paths_by_anchor_token(&db, &anchor_token).await {
        Ok(paths) => {
            info!("Successfully retrieved {} paths", paths.len());
            Ok(HttpResponse::Ok().json(paths))
        }
        Err(e) => {
            error!("Failed to retrieve paths: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve paths: {}",
                e
            )))
        }
    }
}

/// GET /paths/chain/{chain_id} - Returns paths by chain ID
///
/// # Arguments
/// * `db` - Database connection
/// * `path` - Path parameters containing chain_id
///
/// # Returns
/// JSON array of PathResponse objects
pub async fn get_paths_by_chain_id_handler(
    db: web::Data<Database>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let chain_id = path.into_inner();
    info!("Handling GET /paths/chain/{} request", chain_id);

    match PathService::get_paths_by_chain_id(&db, chain_id).await {
        Ok(paths) => {
            info!("Successfully retrieved {} paths", paths.len());
            Ok(HttpResponse::Ok().json(paths))
        }
        Err(e) => {
            error!("Failed to retrieve paths: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to retrieve paths: {}",
                e
            )))
        }
    }
}

/// POST /paths - Creates a new path
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `body` - CreatePathRequest containing path data
///
/// # Returns
/// JSON object of PathResponse containing created path information
pub async fn create_path_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    body: web::Json<CreatePathRequest>,
) -> Result<HttpResponse, ApiError> {
    info!("Handling POST /paths request");

    match PathService::create_path(&db, body.into_inner()).await {
        Ok(path) => {
            info!("Successfully created path with id: {}", path.id);
            Ok(HttpResponse::Created().json(path))
        }
        Err(e) => {
            error!("Failed to create path: {}", e);
            Err(ApiError::DatabaseError(format!(
                "Failed to create path: {}",
                e
            )))
        }
    }
}

/// PUT /paths/{id} - Updates an existing path
/// Requires API key authentication via X-API-Key header
///
/// # Arguments
/// * `_api_key` - API key from X-API-Key header (validated by extractor)
/// * `db` - Database connection
/// * `path` - Path parameters containing id
/// * `body` - UpdatePathRequest containing fields to update
///
/// # Returns
/// JSON object of PathResponse containing updated path information
pub async fn update_path_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<UpdatePathRequest>,
) -> Result<HttpResponse, ApiError> {
    let id_str = path.into_inner();
    info!("Handling PUT /paths/{} request", id_str);

    let id = match ObjectId::parse_str(&id_str) {
        Ok(id) => id,
        Err(e) => {
            error!("Invalid ObjectId format: {}", e);
            return Err(ApiError::BadRequest(format!("Invalid ID format: {}", e)));
        }
    };

    match PathService::update_path(&db, &id, body.into_inner()).await {
        Ok(path) => {
            info!("Successfully updated path with id: {}", id_str);
            Ok(HttpResponse::Ok().json(path))
        }
        Err(e) => {
            error!("Failed to update path {}: {}", id_str, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Path with id {} not found",
                    id_str
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to update path: {}",
                    e
                )))
            }
        }
    }
}

/// DELETE /paths/{id} - Soft deletes a path (sets deleted_at)
/// Requires API key authentication via X-API-Key header
pub async fn delete_path_handler(
    _api_key: ApiKey,
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id_str = path.into_inner();
    info!("Handling DELETE /paths/{} request", id_str);

    let id = match ObjectId::parse_str(&id_str) {
        Ok(id) => id,
        Err(e) => {
            error!("Invalid ObjectId format: {}", e);
            return Err(ApiError::BadRequest(format!("Invalid ID format: {}", e)));
        }
    };

    match PathService::delete_path(&db, &id).await {
        Ok(()) => {
            info!("Successfully soft deleted path with id: {}", id_str);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(e) => {
            error!("Failed to delete path {}: {}", id_str, e);
            if e.to_string().contains("not found") {
                Err(ApiError::NotFound(format!(
                    "Path with id {} not found",
                    id_str
                )))
            } else {
                Err(ApiError::DatabaseError(format!(
                    "Failed to delete path: {}",
                    e
                )))
            }
        }
    }
}
