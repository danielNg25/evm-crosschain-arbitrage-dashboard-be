use actix_web::web;

use crate::handlers::pool::{
    count_pools_by_network_id_handler, create_pool_handler, delete_pool_handler,
    get_pool_by_address_handler, get_pools_by_network_id_handler, get_pools_handler,
    hard_delete_pool_handler, update_pool_handler,
};

pub fn configure_pool_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/pools", web::get().to(get_pools_handler))
        .route("/pools", web::post().to(create_pool_handler))
        .route("/pools/{id}", web::put().to(update_pool_handler))
        .route("/pools/{id}", web::delete().to(delete_pool_handler))
        .route(
            "/pools/{id}/hard",
            web::delete().to(hard_delete_pool_handler),
        )
        .route(
            "/pools/network/{network_id}",
            web::get().to(get_pools_by_network_id_handler),
        )
        .route(
            "/pools/network/{network_id}/address/{address}",
            web::get().to(get_pool_by_address_handler),
        )
        .route(
            "/pools/network/{network_id}/count",
            web::get().to(count_pools_by_network_id_handler),
        );
}
