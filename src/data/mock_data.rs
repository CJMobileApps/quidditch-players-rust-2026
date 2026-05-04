use crate::data::model::house::{House, HouseName};
use crate::data::model::player::Player;
use crate::data::model::position::Position;
use std::collections::HashMap;
use std::sync::LazyLock;
use uuid::Uuid;

pub struct MockData;

impl MockData {

    pub(crate) fn get_all_quidditch_teams() -> &'static Vec<Player> {
        &ALL_QUIDDITCH_TEAMS
    }

    pub(crate) fn get_gryffindor_team() -> &'static Vec<Player> {
        &GRYFFINDOR_TEAM
    }

    pub(crate) fn get_slytherin_team() -> &'static Vec<Player> {
        &SLYTHERIN_TEAM
    }

    pub(crate) fn get_ravenclaw_team() -> &'static Vec<Player> {
        &RAVENCLAW_TEAM
    }

    pub(crate) fn get_hufflepuff_team() -> &'static Vec<Player> {
        &HUFFLEPUFF_TEAM
    }

    pub(crate) fn get_houses() -> &'static Vec<House> {
        &HOUSES
    }

    pub(crate) fn get_positions() -> &'static HashMap<i32, Position> {
        &POSITIONS
    }

    pub(crate) fn get_statuses(name: &str, favorite_subject: &str) -> Vec<String> {
        vec![
            format!("{} is studying {} \u{1F4DA}", name, favorite_subject),
            format!("{} is practicing Quidditch \u{1F9F9}", name),
            format!("{} is eating in the Great Hall \u{1F357}", name),
            format!("{} is serving detention with Mr. Flich \u{1F557}", name),
            format!("{} is drinking butterbeer at Hogsmeade \u{1F37B}", name),
            format!("{} is lost in the Forbidden Forest \u{1F332}", name),
            format!("{} is sleeping in the Gryffindor Dormitory \u{1F634}", name),
            format!("{} is relaxing in the Gryffindor Common Room \u{1F981}", name),
            format!("{} is dueling a Slytherin \u{1F40D}", name),
            format!("{} is destroying a horcrux \u{1F5E1}", name),
            format!("{} is battling a basilisk in the Chamber of Secrets \u{1F92B}", name),
            format!("{} is escaping Azkaban \u{1F6A3}", name),
            format!("{} is breaking into the Ministry of Magic \u{1F52E}", name),
            format!("{} is heading to put name into Goblet of Fire \u{1F525}", name),
        ]
    }
}

static HOUSES: LazyLock<Vec<House>> = LazyLock::new(|| {
    vec![
        House {
            name: HouseName::Gryffindor.to_string(),
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
            emoji: "\u{1F981}".to_string(),
        },
        House {
            name: HouseName::Slytherin.to_string(),
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/6/6e/Slytherin.jpg/revision/latest".to_string(),
            emoji: "\u{1F40D}".to_string(),
        },
        House {
            name: HouseName::Ravenclaw.to_string(),
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/3/3c/RavenclawCrest.jpg/revision/latest".to_string(),
            emoji: "\u{1F985}".to_string(),
        },
        House {
            name: HouseName::Hufflepuff.to_string(),
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/e/e4/Hufflepuff.jpg/revision/latest".to_string(),
            emoji: "\u{1F9A1}".to_string(),
        },
    ]
});

static ALL_QUIDDITCH_TEAMS: LazyLock<Vec<Player>> = LazyLock::new(|| {
    GRYFFINDOR_TEAM.iter()
        .chain(SLYTHERIN_TEAM.iter())
        .chain(RAVENCLAW_TEAM.iter())
        .chain(HUFFLEPUFF_TEAM.iter())
        .cloned()
        .collect()
});

static GRYFFINDOR_TEAM: LazyLock<Vec<Player>> = LazyLock::new(|| {
    vec![
        Player {
            id: Uuid::parse_str("fd1f2deb-9637-4214-b991-a1b8daf18a7b").unwrap(),
            first_name: "Harry".to_string(),
            last_name: "Potter".to_string(),
            years_played: vec![1991, 1992, 1993, 1994, 1995, 1996, 1997],
            favorite_subject: "Defense Against The Dark Arts".to_string(),
            // Seeker
            position: 4,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/c/ce/Harry_Potter_DHF1.jpg/revision/latest".to_string(),
            house_name: HouseName::Gryffindor.to_string(),
        },
        Player {
            id: Uuid::parse_str("ef55fa4b-b88f-4623-aaad-7abdcf2ea4f6").unwrap(),
            first_name: "Katie".to_string(),
            last_name: "Bell".to_string(),
            years_played: vec![1991, 1992, 1993, 1994, 1995, 1996, 1997],
            favorite_subject: "Transfiguration".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/3/32/Katie_Bell_infobox.jpg/revision/latest".to_string(),
            house_name: HouseName::Gryffindor.to_string(),
        },
        Player {
            id: Uuid::parse_str("c2e851f6-9400-4c9f-89aa-936dfc6de90c").unwrap(),
            first_name: "Angelina".to_string(),
            last_name: "Johnson".to_string(),
            years_played: vec![1990, 1991, 1992, 1993, 1994, 1995, 1996],
            favorite_subject: "Care of Magical Creatures".to_string(),
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/2/24/GOF_promo_Angelina_Johnson.jpg/revision/latest".to_string(),
            house_name: HouseName::Gryffindor.to_string(),
        },
        Player {
            id: Uuid::parse_str("841c567f-c8f8-4945-8401-ecb81e7219f2").unwrap(),
            first_name: "Fred".to_string(),
            last_name: "Weasley".to_string(),
            years_played: vec![1990, 1991, 1992, 1993, 1994, 1995, 1996],
            favorite_subject: "Charms".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/9e/Fred_Weasley_promo_DHF1.jpg/revision/latest".to_string(),
            house_name: HouseName::Gryffindor.to_string(),
        },
        Player {
            id: Uuid::parse_str("7891a848-883c-4925-aa43-a6fb620195fa").unwrap(),
            first_name: "George".to_string(),
            last_name: "Weasley".to_string(),
            years_played: vec![1990, 1991, 1992, 1993, 1994, 1995, 1996],
            favorite_subject: "Charms".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/b/b5/George_Weasley_promo_DHF1.jpg/revision/latest".to_string(),
            house_name: HouseName::Gryffindor.to_string(),
        },
        Player {
            id: Uuid::parse_str("b10e1a15-df78-47ab-94b6-78942437b1ad").unwrap(),
            first_name: "Alicia".to_string(),
            last_name: "Spinnet".to_string(),
            years_played: vec![1990, 1991, 1992, 1993, 1994, 1995, 1996],
            favorite_subject: "Charms".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/7/7a/Alicia_Spinnet.png/revision/latest?cb=20160720064800".to_string(),
            house_name: HouseName::Gryffindor.to_string(),
        },
        Player {
            id: Uuid::parse_str("a04e1b6f-9b7f-407e-8beb-aaf7b8d34655").unwrap(),
            first_name: "Oliver".to_string(),
            last_name: "Wood".to_string(),
            years_played: vec![1989, 1990, 1991, 1992, 1993, 1994],
            favorite_subject: "Potions".to_string(),
            // Keeper
            position: 3,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/2/2f/Oliver_WoodDH2.jpg/revision/latest?cb=20110801072255".to_string(),
            house_name: HouseName::Gryffindor.to_string(),
        },
    ]
});

static SLYTHERIN_TEAM: LazyLock<Vec<Player>> = LazyLock::new(|| {
    vec![
        Player {
            id: Uuid::parse_str("f5272335-7f6f-4aea-b0ba-c5c5dcec4aa5").unwrap(),
            first_name: "Draco".to_string(),
            last_name: "Malfoy".to_string(),
            years_played: vec![1992, 1993, 1994, 1995, 1996, 1997],
            favorite_subject: "Potions".to_string(),
            // Seeker
            position: 4,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/7/7e/Draco_Malfoy_TDH.png/revision/latest/scale-to-width-down/1000?cb=20180116013508".to_string(),
            house_name: HouseName::Slytherin.to_string(),
        },
        Player {
            id: Uuid::parse_str("d86096a5-9d9b-4dc6-b830-1de5431a1f37").unwrap(),
            first_name: "Miles".to_string(),
            last_name: "Bletchley".to_string(),
            years_played: vec![1991, 1992, 1993, 1994, 1995, 1996],
            favorite_subject: "Study of Ancient Runes".to_string(),
            // Keeper
            position: 3,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/4/40/Miles_Bletchley.jpeg/revision/latest?cb=20110810003628".to_string(),
            house_name: HouseName::Slytherin.to_string(),
        },
        Player {
            id: Uuid::parse_str("1dd6f764-365f-4013-8bc6-cacab0f45232").unwrap(),
            first_name: "Gregory".to_string(),
            last_name: "Goyle".to_string(),
            years_played: vec![1995, 1996, 1997],
            favorite_subject: "Potions".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/e/e7/Gregory_Goyle_DH2.jpg/revision/latest?cb=20180306163743".to_string(),
            house_name: HouseName::Slytherin.to_string(),
        },
        Player {
            id: Uuid::parse_str("87ca2176-a15e-400e-98c2-21a4b7b34785").unwrap(),
            first_name: "Vincent".to_string(),
            last_name: "Crabbe".to_string(),
            years_played: vec![1995, 1996, 1997],
            favorite_subject: "Potions".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/b/ba/Vincent_Crabbe.jpg/revision/latest/scale-to-width-down/1000?cb=20091224183746".to_string(),
            house_name: HouseName::Slytherin.to_string(),
        },
        Player {
            id: Uuid::parse_str("add49a74-db89-4c8b-bbcb-89be313442f7").unwrap(),
            first_name: "Cassius".to_string(),
            last_name: "Warrington".to_string(),
            years_played: vec![1993, 1994, 1995, 1996],
            favorite_subject: "History of Magic".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/0/08/Cassius_Warrington_OOTPF.bmp/revision/latest/thumbnail/width/360/height/360?cb=20130416151820".to_string(),
            house_name: HouseName::Slytherin.to_string(),
        },
        Player {
            id: Uuid::parse_str("498810f5-e1b1-47ff-865a-22ef7ff72c69").unwrap(),
            first_name: "Adrian".to_string(),
            last_name: "Pucey".to_string(),
            years_played: vec![1995, 1996],
            favorite_subject: "Magical Theory".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/1/13/Adrianpucey-HS.jpg/revision/latest?cb=20101126164937".to_string(),
            house_name: HouseName::Slytherin.to_string(),
        },
        Player {
            id: Uuid::parse_str("627efe48-7a10-45ce-b64c-2027926dd71e").unwrap(),
            first_name: "Graham".to_string(),
            last_name: "Montague".to_string(),
            years_played: vec![1993, 1994, 1995, 1996],
            favorite_subject: "Transfiguration".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/c/c3/Graham_montague.jpg/revision/latest?cb=20140701101409&path-prefix=fr".to_string(),
            house_name: HouseName::Slytherin.to_string(),
        },
    ]
});

static RAVENCLAW_TEAM: LazyLock<Vec<Player>> = LazyLock::new(|| {
    vec![
        Player {
            id: Uuid::parse_str("aa7fb66e-827f-42db-9aac-974c87b35504").unwrap(),
            first_name: "Cho".to_string(),
            last_name: "Chang".to_string(),
            years_played: vec![1993, 1994, 1995, 1996],
            favorite_subject: "Apparition".to_string(),
            // Seeker
            position: 4,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/1/1e/Cho_Chang.jpg/revision/latest?cb=20180322164130".to_string(),
            house_name: HouseName::Ravenclaw.to_string(),
        },
        Player {
            id: Uuid::parse_str("ef968277-e996-4eca-8f94-1928dde4a979").unwrap(),
            first_name: "Grant".to_string(),
            last_name: "Page".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Charms".to_string(),
            // Keeper
            position: 3,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/93/GrantPage.png/revision/latest?cb=20130320232028".to_string(),
            house_name: HouseName::Ravenclaw.to_string(),
        },
        Player {
            id: Uuid::parse_str("cdf95045-8df9-4609-bb26-5d4752823022").unwrap(),
            first_name: "Duncan".to_string(),
            last_name: "Inglebee".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Astronomy".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/2/29/Dinglebee.png/revision/latest?cb=20140827133418".to_string(),
            house_name: HouseName::Ravenclaw.to_string(),
        },
        Player {
            id: Uuid::parse_str("870d5078-584d-4d34-9ff9-303db6c03992").unwrap(),
            first_name: "Jason".to_string(),
            last_name: "Samuels".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Transfiguration".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/1/1b/Jasonsamuelsqwc.png/revision/latest?cb=20140827133708".to_string(),
            house_name: HouseName::Ravenclaw.to_string(),
        },
        Player {
            id: Uuid::parse_str("8726e642-65a9-4dd7-b8eb-08f2a5850f4d").unwrap(),
            first_name: "Randolph".to_string(),
            last_name: "Burrow".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Advanced Arithmancy Studies".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/0/07/RandolphBurrow.png/revision/latest?cb=20130320231816".to_string(),
            house_name: HouseName::Ravenclaw.to_string(),
        },
        Player {
            id: Uuid::parse_str("f8f11664-a932-4e93-b93f-1d8ca4c0cf48").unwrap(),
            first_name: "Jeremy".to_string(),
            last_name: "Stretton".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Alchemy".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/0/06/Jeremy_Stretton_Cleansweep_Seven.jpg/revision/latest?cb=20091020205540".to_string(),
            house_name: HouseName::Ravenclaw.to_string(),
        },
        Player {
            id: Uuid::parse_str("c2fe9d3a-140d-439d-9f15-2f48475eee51").unwrap(),
            first_name: "Roger".to_string(),
            last_name: "Davies".to_string(),
            years_played: vec![1993, 1994, 1995, 1996],
            favorite_subject: "Apparition".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/e/e5/Roger_Davies.jpg/revision/latest?cb=20180322052136".to_string(),
            house_name: HouseName::Ravenclaw.to_string(),
        },
    ]
});

static HUFFLEPUFF_TEAM: LazyLock<Vec<Player>> = LazyLock::new(|| {
    vec![
        Player {
            id: Uuid::parse_str("4f9b53b6-b1c1-42d6-9217-2a0f39f010e3").unwrap(),
            first_name: "Cedric".to_string(),
            last_name: "Diggory".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Charms".to_string(),
            // Seeker
            position: 4,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/90/Cedric_Diggory_Profile.png/revision/latest/scale-to-width-down/1000?cb=20161123045136".to_string(),
            house_name: HouseName::Hufflepuff.to_string(),
        },
        Player {
            id: Uuid::parse_str("fa680863-997d-4213-b9c3-d8839099bcfb").unwrap(),
            first_name: "Herbert".to_string(),
            last_name: "Fleet".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "History of Magic".to_string(),
            // Keeper
            position: 3,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/0/04/Herbert_Fleet.png/revision/latest?cb=20170304124757".to_string(),
            house_name: HouseName::Hufflepuff.to_string(),
        },
        Player {
            id: Uuid::parse_str("aa7fb66e-827f-42db-9aac-974c87b35504").unwrap(),
            first_name: "Anthony".to_string(),
            last_name: "Rickett".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Muggle Music".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harryalbuspotter/images/e/ea/Anthony_Rickett.PNG/revision/latest?cb=20120107004734".to_string(),
            house_name: HouseName::Hufflepuff.to_string(),
        },
        Player {
            id: Uuid::parse_str("adc01148-3d20-4dd6-a421-f87d784e58ac").unwrap(),
            first_name: "Maxine".to_string(),
            last_name: "O'Flaherty".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Muggle Art".to_string(),
            // Beater
            position: 2,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/6/64/Maxine_O%27Flaherty.png/revision/latest?cb=20170304123914".to_string(),
            house_name: HouseName::Hufflepuff.to_string(),
        },
        Player {
            id: Uuid::parse_str("57b2d3d9-23a4-45ca-84ed-eb1154c34c07").unwrap(),
            first_name: "Tamsin".to_string(),
            last_name: "Applebee".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Advanced Arithmancy Studies".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/4/48/Tamsin_Applebee.png/revision/latest?cb=20170304124301".to_string(),
            house_name: HouseName::Hufflepuff.to_string(),
        },
        Player {
            id: Uuid::parse_str("9651d59e-74da-43bd-b738-46a65097959b").unwrap(),
            first_name: "Heidi".to_string(),
            last_name: "Macavoy".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Muggle Art".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/a/af/Heidi_Macavoy.png/revision/latest?cb=20170304123437".to_string(),
            house_name: HouseName::Hufflepuff.to_string(),
        },
        Player {
            id: Uuid::parse_str("757c624c-40e3-4e9f-a4a8-40cd09839c8f").unwrap(),
            first_name: "Malcolm".to_string(),
            last_name: "Preece".to_string(),
            years_played: vec![1993, 1994],
            favorite_subject: "Ghoul Studies".to_string(),
            // Chaser
            position: 1,
            image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/92/Malcolm_Preece.png/revision/latest?cb=20170304122953".to_string(),
            house_name: HouseName::Hufflepuff.to_string(),
        },
    ]
});

static POSITIONS: LazyLock<HashMap<i32, Position>> = LazyLock::new(|| {
    HashMap::from([
        (1, Position { position_name: "Chaser".to_string() }),
        (2, Position { position_name: "Beater".to_string() }),
        (3, Position { position_name: "Keeper".to_string() }),
        (4, Position { position_name: "Seeker".to_string() }),
    ])
});
