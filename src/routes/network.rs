use actix_web::web;

use crate::handlers::network::{
    create_network_handler, delete_network_handler, get_network_by_chain_id_handler,
    get_networks_handler, update_factories_handler, update_network_handler,
};

pub fn configure_network_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/networks", web::get().to(get_networks_handler))
        .route("/networks", web::post().to(create_network_handler))
        .route(
            "/networks/{chain_id}",
            web::get().to(get_network_by_chain_id_handler),
        )
        .route(
            "/networks/{chain_id}",
            web::put().to(update_network_handler),
        )
        .route(
            "/networks/{chain_id}",
            web::delete().to(delete_network_handler),
        )
        .route(
            "/networks/{chain_id}/factories",
            web::put().to(update_factories_handler),
        );
}
