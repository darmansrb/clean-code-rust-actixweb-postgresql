use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Car {
    pub id: Uuid,
    pub model: String,
    pub year: i32,
    pub price_per_day: Decimal,
    pub is_available: bool,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCarDto {
    pub model: String,
    pub year: i32,
    pub price_per_day: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCarDto {
    pub model: Option<String>,
    pub year: Option<i32>,
    pub price_per_day: Option<Decimal>,
    pub is_available: Option<bool>,
}