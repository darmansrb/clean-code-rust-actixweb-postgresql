use actix_web::{web, HttpResponse};
use crate::models::car::{Car, CreateCarDto, UpdateCarDto};
use crate::services::car::CarService;
use crate::models::errors::AppError;
use log::{info, error};
use uuid::Uuid;

pub async fn create(
  service: web::Data<CarService>,
  dto: web::Json<CreateCarDto>,
) -> Result<HttpResponse, actix_web::Error> {
  let car = service
      .create(dto.into_inner())
      .await
      .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

  Ok(HttpResponse::Created().json(car))
}

pub async fn find_all(
  service: web::Data<CarService>,
) -> Result<HttpResponse, actix_web::Error> {
  let cars = service
      .find_all()
      .await
      .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

  Ok(HttpResponse::Ok().json(cars))
}

pub async fn find_by_id(
    service: web::Data<CarService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let car = service
        .find_by_id(id.into_inner())
        .await
        .map_err(|e| match e {
            AppError::NotFound => actix_web::error::ErrorNotFound(e),
            _ => actix_web::error::ErrorInternalServerError(e),
        })?;

    Ok(HttpResponse::Ok().json(car))
}

pub async fn update(
    service: web::Data<CarService>,
    id: web::Path<Uuid>,
    dto: web::Json<UpdateCarDto>,
) -> Result<HttpResponse, actix_web::Error> {
    let car = service
        .update(id.into_inner(), dto.into_inner())
        .await
        .map_err(|e| match e {
            AppError::NotFound => actix_web::error::ErrorNotFound(e),
            AppError::Validation(_) => actix_web::error::ErrorBadRequest(e),
            _ => actix_web::error::ErrorInternalServerError(e),
        })?;

    Ok(HttpResponse::Ok().json(car))
}

pub async fn delete(
  service: web::Data<CarService>,
  id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
  service
      .delete(id.into_inner())
      .await
      .map_err(|e| match e {
          AppError::NotFound => actix_web::error::ErrorNotFound(e),
          _ => actix_web::error::ErrorInternalServerError(e),
      })?;

  Ok(HttpResponse::NoContent().finish())
}