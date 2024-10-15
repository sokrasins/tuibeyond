
pub mod character {

    // JSON object parts needed for parsing
    use crate::dnd_json::dnd_json::CharacterJson;
    use crate::dnd_json::dnd_json::ItemElement;
    use crate::srd::srd;

    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Character {
        pub name: String,
        pub level: i64,
        pub race: String,
        pub class: String,
        pub ability_scores: HashMap<srd::AbilityType, i64>,
        pub skill_profs: Vec<srd::SkillType>,
        pub saving_throw_profs: Vec<srd::AbilityType>,
    }

    impl Character {
        pub fn new() -> Character {
            Character {
                name: "".to_string(),
                level: 0,
                race: "".to_string(),
                class: "".to_string(),
                ability_scores: HashMap::new(),
                skill_profs: Vec::new(),
                saving_throw_profs: Vec::new(),
            }
        }

        pub fn from_json(json: &CharacterJson) -> Character {
            // Read base ability score values
            // TODO: We're not checking for anything in items that may alter ability scores
            let mut ability_scores = HashMap::from([
                (srd::AbilityType::Strength, 0),
                (srd::AbilityType::Dexterity, 0),
                (srd::AbilityType::Constitution, 0),
                (srd::AbilityType::Intelligence, 0),
                (srd::AbilityType::Wisdom, 0),
                (srd::AbilityType::Charisma, 0),
            ]);

            // Insert base stats into hashmap
            for (i, stat) in json.data.stats.iter().enumerate() {
                let score = ability_scores.get_mut(
                    &srd::AbilityType::from_u32(i as u32)
                ).unwrap();
                *score = stat.value.unwrap();
            }

            // Find ASIs
            let mut asi = Vec::new();
            Self::find_ability_modifier(&json.data.modifiers.race, &mut asi);
            Self::find_ability_modifier(&json.data.modifiers.class, &mut asi);

            // Add ASIs to stats
            for elt in asi.iter() {
                let score = ability_scores.get_mut(elt).unwrap(); 
                *score += 1;
            }

            // Find skill proficiencies
            let mut skill_profs = Vec::new();
            Self::find_skill_profs(&json.data.modifiers.race, &mut skill_profs);
            Self::find_skill_profs(&json.data.modifiers.class, &mut skill_profs);
            Self::find_skill_profs(&json.data.modifiers.background, &mut skill_profs);

            // Get saving throw proficiencies
            let mut saving_throw_profs = Vec::new();
            Self::find_saving_throw_profs(&json.data.modifiers.class, &mut saving_throw_profs);

            // Done!
            Character {
                name: json.data.name.to_owned(),
                level: json.data.classes[0].level,
                race: json.data.race.base_race_name.to_owned(),
                class: json.data.classes[0].definition.name.to_owned(),
                ability_scores,
                skill_profs,
                saving_throw_profs,
            }
        }

        fn find_ability_modifier(mods: &Vec<ItemElement>, out: &mut Vec<srd::AbilityType>) {
            // Build map from ability score string to stat vector index
            let ability_score_map: HashMap<String, srd::AbilityType> = HashMap::from([
                ("strength-score".to_string(),      srd::AbilityType::Strength),
                ("dexterity-score".to_string(),     srd::AbilityType::Dexterity),
                ("constitution-score".to_string(),  srd::AbilityType::Constitution),
                ("intelligence-score".to_string(),  srd::AbilityType::Intelligence),
                ("wisdom-score".to_string(),        srd::AbilityType::Wisdom),
                ("charisma-score".to_string(),      srd::AbilityType::Charisma)
            ]);

            for item in mods.iter() {
                if item.background_type == "bonus" {
                    match ability_score_map.get(&item.sub_type) {
                        Some(ability) => out.push(ability.clone()),
                        None => ()
                    }
                }
            }
        }

        fn find_skill_profs(mods: &Vec<ItemElement>, out: &mut Vec<srd::SkillType>) {
            let skill_map: HashMap<String, srd::SkillType> = HashMap::from([
                ("acrobatics".to_string(), srd::SkillType::Acrobatics),
                ("animal-handling".to_string(), srd::SkillType::AnimalHandling),
                ("arcana".to_string(), srd::SkillType::Arcana),
                ("athletics".to_string(), srd::SkillType::Athletics),
                ("deception".to_string(), srd::SkillType::Deception),
                ("history".to_string(), srd::SkillType::History),
                ("insight".to_string(), srd::SkillType::Insight),
                ("investigation".to_string(), srd::SkillType::Investigation),
                ("intimidation".to_string(), srd::SkillType::Intimidation),
                ("medicine".to_string(), srd::SkillType::Medicine),
                ("nature".to_string(), srd::SkillType::Nature),
                ("perception".to_string(), srd::SkillType::Perception),
                ("performance".to_string(), srd::SkillType::Performance),
                ("persuation".to_string(), srd::SkillType::Persuation),
                ("religion".to_string(), srd::SkillType::Religion),
                ("sleight-of-hand".to_string(), srd::SkillType::SleightOfHand),
                ("stealth".to_string(), srd::SkillType::Stealth),
                ("survival".to_string(), srd::SkillType::Survival),
            ]);

            for item in mods.iter() {
                if item.background_type == "proficiency" {
                    match skill_map.get(&item.sub_type) {
                        Some(ability) => out.push(ability.clone()),
                        None => ()
                    }
                }
            }
        }

        fn find_saving_throw_profs(mods: &Vec<ItemElement>, out: &mut Vec<srd::AbilityType>) {
            // Build map from ability score string to stat vector index
            let ability_score_map: HashMap<String, srd::AbilityType> = HashMap::from([
                ("strength-saving-throws".to_string(),      srd::AbilityType::Strength),
                ("dexterity-saving-throws".to_string(),     srd::AbilityType::Dexterity),
                ("constitution-saving-throws".to_string(),  srd::AbilityType::Constitution),
                ("intelligence-saving-throws".to_string(),  srd::AbilityType::Intelligence),
                ("wisdom-saving-throws".to_string(),        srd::AbilityType::Wisdom),
                ("charisma-saving-throws".to_string(),      srd::AbilityType::Charisma)
            ]);

            for item in mods.iter() {
                if item.background_type == "proficiency" {
                    match ability_score_map.get(&item.sub_type) {
                        Some(ability) => out.push(ability.clone()),
                        None => ()
                    }
                }
            }
        }
    }
}

