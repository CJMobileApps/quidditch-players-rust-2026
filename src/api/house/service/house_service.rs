use std::sync::Arc;
use crate::api::house::repository::house_repository::HouseRepository;
use crate::data::model::house::House;
use crate::util::error::QuidditchPlayersError;

pub trait HouseService: Send + Sync {
    fn get_all_houses(&self) -> Result<Vec<House>, QuidditchPlayersError>;
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
    fn get_all_houses(&self) -> Result<Vec<House>, QuidditchPlayersError> {
        self.user_repository.get_all_houses()
    }
}
