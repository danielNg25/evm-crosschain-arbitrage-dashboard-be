use actix_web::web;

use crate::handlers::path::{
    create_path_handler, get_path_by_id_handler, get_paths_by_anchor_token_handler,
    get_paths_by_chain_id_handler, get_paths_handler, update_path_handler,
};

pub fn configure_path_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/paths", web::get().to(get_paths_handler))
        .route("/paths", web::post().to(create_path_handler))
        .route("/paths/{id}", web::get().to(get_path_by_id_handler))
        .route("/paths/{id}", web::put().to(update_path_handler))
        .route(
            "/paths/anchor-token/{anchor_token}",
            web::get().to(get_paths_by_anchor_token_handler),
        )
        .route(
            "/paths/chain/{chain_id}",
            web::get().to(get_paths_by_chain_id_handler),
        );
}
