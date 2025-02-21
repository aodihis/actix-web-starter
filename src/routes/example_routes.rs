use actix_web::web;
use crate::handlers::api::example_handler::example_handler;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/example", web::post().to(example_handler)));
}
