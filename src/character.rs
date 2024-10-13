use crate::dnd_json::CharacterJson;

pub mod character {
    #[derive(Debug)]
    pub struct Character {
        pub name: String,
        pub level: i64,
        pub race: String,
        pub class: String,
        pub ability_scores: [i64; 6],
    }


    impl Character {
        pub fn new() -> Character {
            Character {
                name: "".to_string(),
                level: 0,
                race: "".to_string(),
                class: "".to_string(),
                ability_scores: [0, 0, 0, 0, 0, 0],
            }
        }

        pub fn from_json(&json: CharacterJson) -> Character {
            Character {
                name: "".to_string(),
                level: 0,
                race: "".to_string(),
                class: "".to_string(),
                ability_scores: [0, 0, 0, 0, 0, 0],
            }
        }
    }
}

