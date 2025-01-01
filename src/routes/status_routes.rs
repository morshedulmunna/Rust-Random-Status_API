use crate::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/status/random", web::get().to(handlers::get_random_status))
        .route("/status/add", web::post().to(handlers::add_status))
        .route("/status/all", web::get().to(handlers::get_all_statuses));
}
