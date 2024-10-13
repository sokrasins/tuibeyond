use std::io;
use std::collections::HashMap;
use tuibeyond::Character;
use tuibeyond::ChoiceDefinition;
use tuibeyond::FeatElement;

mod sheet;
use crate::sheet::sheet;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut api_url: String = "https://character-service.dndbeyond.com/character/v5/character/".to_owned();

    println!("D&DBeyond Character sheet, v.0.0.0");

    println!("Please enter the URL of your character sheet (make sure it's public):");

    // Test url for Arlo
    //let char_url = "https://www.dndbeyond.com/characters/88796596";
    let mut char_url = String::new();

    io::stdin()
        .read_line(&mut char_url)
        .expect("Failed to read character ID");

    // TODO: Do some sanity checking on the value here. How is a character id formatted?
    let char_id = char_url.split("/").last().unwrap().trim();
    
    api_url.push_str(char_id);
    println!("Your character id: {:?}", char_id);
    println!("  Character API URL: {:?}", api_url);

    // Parse response to text
    let resp = reqwest::get(api_url)
        .await?
        .text()
        .await?;

    // Display entire response. It's huge!
    //println!("{:?}", resp);

    // Parse text to struct 
    let char: Character = serde_json::from_str(&resp)?;

    println!("Found your character: {}, a level {} {} {}",
        char.data.name, 
        char.data.classes[0].level,
        char.data.race.base_race_name,
        char.data.classes[0].definition.name);

    // Build map from ability score string to stat vector index
    let mut ability_score_map: HashMap<String, usize> = HashMap::new();
    ability_score_map.insert(
        "Strength Score".to_string(), 0
    );

    ability_score_map.insert(
        "Dexterity Score".to_string(), 1
    );

    ability_score_map.insert(
        "Constitution Score".to_string(), 2
    );

    ability_score_map.insert(
        "Intelligence Score".to_string(), 3
    );

    ability_score_map.insert(
        "Wisdom Score".to_string(), 4
    );

    ability_score_map.insert(
        "Charisma Score".to_string(), 5
    );

    // Accumulate stats
    let mut stats = Vec::new();
    for stat in char.data.stats.iter() {
       stats.push(stat.value.unwrap());
    }

    // // TODO: Include into stats?
    // for x in char.data.bonus_stats.iter() {
    //     println!("{:?}", x.value);
    // }
    // // TODO: Include into stats?
    // for x in char.data.override_stats.iter() {
    //     println!("{:?}", x.value);
    // }

    // Find all ability score increases
    let mut asi = Vec::new();
    find_asi(&char.data.choices.race, &char.data.choices.choice_definitions, &mut asi);
    find_asi(&char.data.choices.class, &char.data.choices.choice_definitions, &mut asi);

    // Add ASIs to stats
    for elt in asi.iter() {
        match ability_score_map.get(elt.to_owned()) {
           Some(x) => stats[*x] += 1,
           None => println!("No index found for ASI")
        }; 

    }

    // Print stats
    println!("Str: {:?}  Dex: {:?}  Con: {:?}  Int: {:?}  Wis: {:?}  Cha: {:?}", 
        stats[0], stats[1], stats[2], stats[3], stats[4], stats[5]
    );

    Ok(())
}

fn get_asi_choice(choice: i64, defs: &Vec<ChoiceDefinition>) -> Option<&str> {
    for i in defs.iter() {
        for j in i.options.iter() {
            if j.id == choice {
                return Some(&j.label)
            }
        }
    }
    None
}

fn find_asi<'a>(feats: &Vec<FeatElement>, defs: &'a Vec<ChoiceDefinition>, asi: &mut Vec<&'a str>) {
    for elt in feats.iter() {
        if elt.background_type == 2 && elt.sub_type.unwrap() == 5 {
            let asi_name = get_asi_choice(
                elt.option_value, 
                defs
            ).unwrap();
            asi.push(asi_name);
        }
    }
}
