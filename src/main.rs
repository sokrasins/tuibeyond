//use std::io;
use tuibeyond::character::character::Character;
use tuibeyond::dnd_json::dnd_json::CharacterJson;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut api_url: String = "https://character-service.dndbeyond.com/character/v5/character/".to_owned();

    println!("D&DBeyond Character sheet, v.0.0.0");

    println!("Please enter the URL of your character sheet (make sure it's public):");

    // Test url for Arlo
    let char_url = "https://www.dndbeyond.com/characters/88796596";
    // let mut char_url = String::new();

    // io::stdin()
    //     .read_line(&mut char_url)
    //     .expect("Failed to read character ID");

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
    
    let json: CharacterJson = serde_json::from_str(&resp)?;
    let char: Character = Character::from_json(&json);

    println!("{:?}", char);
    
    Ok(())
}


