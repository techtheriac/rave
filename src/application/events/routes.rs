use actix_web::web;
use super::handlers::*;

pub fn event_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/events"))
  .route("/organize", web::post().to(organize_event))
}