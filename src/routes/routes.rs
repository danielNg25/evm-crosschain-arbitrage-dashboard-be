use actix_web::{web, HttpResponse, Result};

use crate::routes::{
    config::configure_config_routes, network::configure_network_routes,
    path::configure_path_routes, pool::configure_pool_routes, token::configure_token_routes,
};

/// Health check endpoint
async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"})))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health_check))
            .configure(configure_config_routes)
            .configure(configure_network_routes)
            .configure(configure_path_routes)
            .configure(configure_pool_routes)
            .configure(configure_token_routes),
    );
}
