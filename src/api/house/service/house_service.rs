use std::sync::Arc;
use crate::api::house::controller::house_controller::AppError;
use crate::api::house::repository::house_repository::HouseRepository;
use crate::data::model::house::{House, HouseName};

pub trait HouseService: Send + Sync {
    fn get_all_houses(&self) -> Result<Vec<House>, AppError>;
}

pub struct HouseServiceImpl {
    user_repository: Arc<dyn HouseRepository>,
}

impl HouseServiceImpl {
    pub fn new(user_repository: Arc<dyn HouseRepository>) -> Self {
        Self { user_repository }
    }
}

impl HouseService for HouseServiceImpl {
    fn get_all_houses(&self) -> Result<Vec<House>, AppError> {
        self.user_repository.get_all_houses()
    }
}

// interface HouseService {
// @Throws(ClientException::class, InternalException::class)
// suspend fun getAllHouses(): List<House>
// }