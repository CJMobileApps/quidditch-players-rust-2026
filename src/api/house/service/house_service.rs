use std::sync::Arc;
use crate::api::house::repository::house_repository::HouseRepository;
use crate::data::model::house::House;
use crate::util::error::QuidditchPlayersError;

pub trait HouseService: Send + Sync {
    fn get_all_houses(&self) -> Result<&[House], QuidditchPlayersError>;
}

pub struct HouseServiceImpl {
    house_repository: Arc<dyn HouseRepository>,
}

impl HouseServiceImpl {
    pub fn new(house_repository: Arc<dyn HouseRepository>) -> Self {
        Self { house_repository }
    }
}

impl HouseService for HouseServiceImpl {
    fn get_all_houses(&self) -> Result<&[House], QuidditchPlayersError> {
        self.house_repository.get_all_houses()
    }
}
