use actix_web::web;

use crate::handlers::config::{get_config_handler, update_config_handler};

pub fn configure_config_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/config", web::get().to(get_config_handler))
        .route("/config", web::put().to(update_config_handler));
}
