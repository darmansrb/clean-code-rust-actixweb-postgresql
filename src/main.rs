use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod config;
mod models;
mod routes;
mod repositories;
mod services;
mod handlers; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Gunakan config database
    let pool = config::database::configure_database().await;

    let car_service = web::Data::new(services::car::CarService::new(
        repositories::car::CarRepository::new(pool.clone())
    ));

    // let user_service = web::Data::new(services::user::UserService::new(
    //     repositories::user::UserRepository::new(pool.clone())
    // ));

    HttpServer::new(move || {
        App::new()
            .app_data(car_service.clone())
            // .app_data(user_service.clone())
            .configure(routes::configure_routes)
    })
    // .workers(4)  // Ganti jumlah worker sesuai kebutuhan, jika di nonaktifkan maka akan menggunakan jumlah core yang ada di CPU secara default
    .bind("127.0.0.1:8080")?
    .run()
    .await
}