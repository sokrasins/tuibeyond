
pub mod character {

    // JSON object parts needed for parsing
    use crate::dnd_json::dnd_json::CharacterJson;
    use crate::dnd_json::dnd_json::ItemElement;
    use crate::srd::srd;

    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Item {
        pub name: String,
        pub equipped: bool,
    }

    #[derive(Debug)]
    pub struct Character {
        pub name: String,
        pub level: i64,
        pub race: String,
        pub class: String,
        pub hit_points: i64,
        pub hit_dice: i64,
        pub ability_scores: HashMap<srd::AbilityType, i64>,
        pub skill_profs: Vec<srd::SkillType>,
        pub saving_throw_profs: Vec<srd::AbilityType>,
        pub inventory: Vec<Item>,
    }

    impl Character {
        pub fn new() -> Character {
            Character {
                name: "".to_string(),
                level: 0,
                race: "".to_string(),
                class: "".to_string(),
                hit_points: 0,
                hit_dice: 0,
                ability_scores: HashMap::new(),
                skill_profs: Vec::new(),
                saving_throw_profs: Vec::new(),
                inventory: Vec::new(),
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
            
            // TODO: Languages, weapons, armor, tools...

            // Level
            let level = json.data.classes[0].level;

            // Calculate hit points
            let con = *ability_scores.get(&srd::AbilityType::Constitution).unwrap();
            let con_mod = srd::AbilityType::get_mod(con);
            let hit_points = json.data.base_hit_points + (con_mod * level);

            // Build item list
            // TODO: See if these items do anything that we can add to our stats
            let mut inventory = Vec::new();
            for item in json.data.inventory.iter() {
                inventory.push(Item {
                    name: item.definition.name.to_owned(),
                    equipped: item.equipped,
                });            
            }

            // Done!
            Character {
                name: json.data.name.to_owned(),
                level,
                race: json.data.race.base_race_name.to_owned(),
                class: json.data.classes[0].definition.name.to_owned(),
                hit_points,
                hit_dice: level,
                ability_scores,
                skill_profs,
                saving_throw_profs,
                inventory,
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

