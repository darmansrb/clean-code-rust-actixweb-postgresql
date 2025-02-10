use crate::models::car::{Car, CreateCarDto, UpdateCarDto};
use crate::models::errors::AppError;
use sqlx::{PgPool, postgres::PgPoolOptions};
use chrono::{DateTime, Utc};
use uuid::Uuid;
// use time::OffsetDateTime;

#[derive(Clone)]
pub struct CarRepository {
    pool: PgPool,
}

impl CarRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateCarDto) -> Result<Car, AppError> {
        let car = sqlx::query_as!(
            Car,
            r#"
            INSERT INTO cars (model, year, price_per_day)
            VALUES ($1, $2, $3)
            RETURNING id, model, year, price_per_day, is_available
            "#,
            dto.model,
            dto.year,
            dto.price_per_day,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(car)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Car, AppError> {
        let car = sqlx::query_as!(
            Car,
            r#"
            SELECT id, model, year, price_per_day, is_available
            FROM cars
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound)?;

        Ok(car)
    }

    pub async fn find_all(&self) -> Result<Vec<Car>, AppError> {
        let cars = sqlx::query_as!(
            Car,
            r#"
            SELECT id, model, year, price_per_day, is_available
            FROM cars
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(cars)
    }

    pub async fn update(&self, id: Uuid, dto: UpdateCarDto) -> Result<Car, AppError> {
        let current = self.find_by_id(id).await?;

        let car = sqlx::query_as!(
            Car,
            r#"
            UPDATE cars
            SET 
                model = $1,
                year = $2,
                price_per_day = $3,
                is_available = $4
            WHERE id = $5
            RETURNING id, model, year, price_per_day, is_available
            "#,
            dto.model.unwrap_or(current.model),
            dto.year.unwrap_or(current.year),
            dto.price_per_day.unwrap_or(current.price_per_day),
            dto.is_available.unwrap_or(current.is_available),
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(car)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM cars
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}