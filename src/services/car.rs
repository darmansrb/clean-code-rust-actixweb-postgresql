use crate::models::car::{Car, CreateCarDto, UpdateCarDto};
use crate::models::errors::AppError;
use crate::repositories::car::CarRepository;
use uuid::Uuid;
use log::{info, error};

#[derive(Clone)]
pub struct CarService {
    repository: CarRepository,
}

impl CarService {
    pub fn new(repository: CarRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, dto: CreateCarDto) -> Result<Car, AppError> {
        print!("service create");
        if dto.year < 1900 {
          info!("Tahun harus di atas 1900");
            return Err(AppError::Validation("Year must be after 1900".to_string()));
        }
        self.repository.create(dto).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Car, AppError> {
        self.repository.find_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<Car>, AppError> {
        self.repository.find_all().await
    }

    pub async fn update(&self, id: Uuid, dto: UpdateCarDto) -> Result<Car, AppError> {
        if let Some(year) = dto.year {
            if year < 1900 {
                return Err(AppError::Validation("Year must be after 1900".to_string()));
            }
        }
        self.repository.update(id, dto).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await
    }
}