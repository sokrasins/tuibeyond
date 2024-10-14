//use std::io;
use tuibeyond::dnd_json::dnd_json::CharacterJson;
use tuibeyond::character::character::Character;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut api_url: String = "https://character-service.dndbeyond.com/character/v5/character/".to_owned();

    println!("D&DBeyond Character sheet, v.0.0.0");

    println!("Please enter the URL of your character sheet (make sure it's public):");

    //let char_url = "https://www.dndbeyond.com/characters/88796596"; // Arlo
    let char_url = "https://www.dndbeyond.com/characters/132884756"; // Mo
    // let mut char_url = String::new();

    // io::stdin()
    //     .read_line(&mut char_url)
    //     .expect("Failed to read character ID");

    // TODO: Do some sanity checking on the value here. How is a character id formatted?
    let char_id = char_url.split("/").last().unwrap().trim();
    api_url.push_str(char_id);

    // Parse response to text
    let resp = reqwest::get(api_url)
        .await?
        .text()
        .await?;

    // Parse text to struct to character
    let json: CharacterJson = serde_json::from_str(&resp)?;
    let char: Character = Character::from_json(&json);

    println!("{:?}", char);

    Ok(())
}

