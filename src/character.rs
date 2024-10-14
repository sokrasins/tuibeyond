
pub mod character {

    // JSON object parts needed for parsing
    use crate::dnd_json::dnd_json::CharacterJson;
    use crate::dnd_json::dnd_json::ChoiceDefinition;
    use crate::dnd_json::dnd_json::FeatElement;
    use crate::srd::srd;

    use std::collections::HashMap;
    use strum::IntoEnumIterator;

    #[derive(Debug)]
    pub struct Character {
        pub name: String,
        pub level: i64,
        pub race: String,
        pub class: String,
        pub ability_scores: HashMap<srd::AbilityType, i64>,
        pub skill_profs: Vec<srd::SkillType>,
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
            }
        }

        pub fn from_json(json: &CharacterJson) -> Character {
            // Read base ability score values
            // TODO: We're not checking for anything in items that may alter ability scores
            let mut stats = Vec::new();
            for stat in json.data.stats.iter() {
                stats.push(stat.value.unwrap());
            }

            // Check if there's non-null information in these
            for x in json.data.bonus_stats.iter() {
                match x.value {
                    Some(val) => println!("TODO: Handle bonus stat: {:?}", val),
                    None => ()
                };
            }
            for x in json.data.override_stats.iter() {
                match x.value {
                    Some(val) => println!("TODO: Handle override stat: {:?}", val),
                    None => (),
                };
            }

            // Find all ability score increases in choices
            let mut asi = Vec::new();
            Self::find_asi(&json.data.choices.race, &json.data.choices.choice_definitions, &mut asi);
            Self::find_asi(&json.data.choices.class, &json.data.choices.choice_definitions, &mut asi);
            Self::find_asi(&json.data.choices.background, &json.data.choices.choice_definitions, &mut asi);
            Self::find_asi(&json.data.choices.feat, &json.data.choices.choice_definitions, &mut asi);

            let mut skill = Vec::new();
            Self::find_skill(&json.data.choices.race, &json.data.choices.choice_definitions, &mut skill);
            Self::find_skill(&json.data.choices.class, &json.data.choices.choice_definitions, &mut skill);
            Self::find_skill(&json.data.choices.background, &json.data.choices.choice_definitions, &mut skill);
            Self::find_skill(&json.data.choices.feat, &json.data.choices.choice_definitions, &mut skill);

            // Stuff skills into a vec
            let mut skill_map: HashMap<String, srd::SkillType> = HashMap::new();
            for skill in srd::SkillType::iter() {
                skill_map.insert(skill.to_string(), skill);
            }

            // Map the skill strings to the SkillType type
            let mut skill_profs = Vec::new();
            for x in skill.iter() {
                skill_profs.push(skill_map.get(*x).unwrap().clone());
            }

            // Build map from ability score string to stat vector index
            let ability_score_map: HashMap<String, srd::AbilityType> = HashMap::from([
                ("Strength Score".to_string(),      srd::AbilityType::Strength),
                ("Dexterity Score".to_string(),     srd::AbilityType::Dexterity),
                ("Constitution Score".to_string(),  srd::AbilityType::Constitution),
                ("Intelligence Score".to_string(),  srd::AbilityType::Intelligence),
                ("Wisdom Score".to_string(),        srd::AbilityType::Wisdom),
                ("Charisma Score".to_string(),      srd::AbilityType::Charisma)
            ]);

            // Add ASIs to stats
            for elt in asi.iter() {
                match ability_score_map.get(elt.to_owned()) {
                    Some(x) => stats[x.clone() as usize] += 1,
                    None => println!("No index found for ASI")
                }; 
            }

            // Built ability score hashmap
            let ability_scores = HashMap::from([
                (srd::AbilityType::Strength,     stats[0]),
                (srd::AbilityType::Dexterity,    stats[1]),
                (srd::AbilityType::Constitution, stats[2]),
                (srd::AbilityType::Intelligence, stats[3]),
                (srd::AbilityType::Wisdom,       stats[4]),
                (srd::AbilityType::Charisma,     stats[5])
            ]);

            // Done!
            Character {
                name: json.data.name.to_owned(),
                level: json.data.classes[0].level,
                race: json.data.race.base_race_name.to_owned(),
                class: json.data.classes[0].definition.name.to_owned(),
                ability_scores,
                skill_profs,
            }
        }

        fn find_asi<'a>(feats: &Vec<FeatElement>, defs: &'a Vec<ChoiceDefinition>, asi: &mut Vec<&'a str>) {
            for elt in feats.iter() {
                // Magic combo of type and subtype that indicate an ASI choice
                if elt.background_type == 2 && elt.sub_type.unwrap() == 5 {
                    let asi_name = Self::get_choice_def_match(
                        elt.option_value, 
                        defs
                    ).unwrap();
                    asi.push(asi_name);
                }
            }
        }

        fn find_skill<'a>(feats: &Vec<FeatElement>, defs: &'a Vec<ChoiceDefinition>, asi: &mut Vec<&'a str>) {
            for elt in feats.iter() {
                // Magic combo of type and subtype that indicate a skill choice
                if elt.background_type == 2 && elt.sub_type.unwrap() == 1 {
                    let asi_name = Self::get_choice_def_match(
                        elt.option_value, 
                        defs
                    ).unwrap();
                    asi.push(asi_name);
                }
            }
        }

        fn get_choice_def_match(choice: i64, defs: &Vec<ChoiceDefinition>) -> Option<&str> {
            for i in defs.iter() {
                for j in i.options.iter() {
                    if j.id == choice {
                        return Some(&j.label)
                    }
                }
            }
            None
        }
    }
}

