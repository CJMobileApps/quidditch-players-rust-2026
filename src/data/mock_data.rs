use crate::data::model::house::{House, HouseName};
use std::sync::LazyLock;

pub struct MockData;

impl MockData {
    pub(crate) fn get_houses() -> &'static Vec<House> {
        &HOUSES
    }
}

static HOUSES: LazyLock<Vec<House>> = LazyLock::new(|| {
    vec![
        House {
            name: HouseName::Gryffindor,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
            emoji: "\u{1F981}".to_string(),
        },
        House {
            name: HouseName::Slytherin,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/6/6e/Slytherin.jpg/revision/latest".to_string(),
            emoji: "\u{1F40D}".to_string(),
        },
        House {
            name: HouseName::Ravenclaw,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/3/3c/RavenclawCrest.jpg/revision/latest".to_string(),
            emoji: "\u{1F985}".to_string(),
        },
        House {
            name: HouseName::Hufflepuff,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/e/e4/Hufflepuff.jpg/revision/latest".to_string(),
            emoji: "\u{1F9A1}".to_string(),
        },
    ]
});
