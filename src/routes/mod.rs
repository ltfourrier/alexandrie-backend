use actix_web::web;

pub mod health;
pub mod users;

pub fn v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(health::get_health)
            .configure(users::init),
    );
}
