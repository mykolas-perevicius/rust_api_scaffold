pub mod health;
pub mod users;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::scope())
       .service(users::scope());
}
