use actix_web::web;
use super::handler::*;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/health", web::get().to(health_check_handler));
}