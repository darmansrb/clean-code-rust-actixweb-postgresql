use actix_web::web;
mod car;
// mod user;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(car::car_routes())
            // .service(user::user_routes())
    );
}