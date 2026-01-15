use actix_web::web;

use crate::handlers::token::{
    count_tokens_by_network_id_handler, delete_token_by_address_handler,
    get_token_by_address_handler, get_tokens_by_network_id_handler, get_tokens_handler,
};

pub fn configure_token_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/tokens", web::get().to(get_tokens_handler))
        .route(
            "/tokens/network/{network_id}",
            web::get().to(get_tokens_by_network_id_handler),
        )
        .route(
            "/tokens/network/{network_id}/address/{address}",
            web::get().to(get_token_by_address_handler),
        )
        .route(
            "/tokens/network/{network_id}/address/{address}",
            web::delete().to(delete_token_by_address_handler),
        )
        .route(
            "/tokens/network/{network_id}/count",
            web::get().to(count_tokens_by_network_id_handler),
        );
}
