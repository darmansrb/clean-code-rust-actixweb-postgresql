use actix_web::{web, HttpResponse, Scope};
use uuid::Uuid;
use crate::models::car::{Car, CreateCarDto, UpdateCarDto};
use crate::models::errors::AppError;
use crate::services::car::CarService;
use crate::handlers::car::{create, find_all, find_by_id, update, delete};

pub fn car_routes() -> Scope {
    web::scope("/cars")
        .service(
            web::resource("")
                .route(web::get().to(find_all))
                .route(web::post().to(create)),
        )
        .service(
            web::resource("/{id}")
                .route(web::get().to(find_by_id))
                .route(web::put().to(update))
                .route(web::delete().to(delete)),
        )
}