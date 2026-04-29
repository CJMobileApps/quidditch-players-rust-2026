use std::sync::LazyLock;
use crate::data::model::house::{House, HouseName};

pub struct MockData;

impl MockData {
    
    pub(crate) fn get_houses() -> &'static Vec<House> {
       &HOUSES
    }

   
}

static HOUSES: LazyLock<Vec<House>> = LazyLock::new(|| {
    let gryffindor_house = House {
        name: HouseName::Gryffindor,
        image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
        emoji: "\u{1F981}".to_string(),
    };
    
    vec![gryffindor_house]
});


