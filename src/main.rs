mod models;
use crate::models::bosses::Boss;
use crate::models::player::Player;
use dialoguer::Error;
use serde::Deserialize;
use serde::Serialize;

use ::std::fs;
use std::arch::x86_64::_CMP_FALSE_OQ;
use std::fs::File;

use dialoguer::Input;
use dialoguer::Select;

fn check_saves() -> std::io::Result<bool> {
    let file_path = "saves/player.json";

    match fs::exists(file_path) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            eprintln!("Error checking for save files: {}", e);
            Err(e)
        }
    }
}

fn main() {
    let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];

    match check_saves() {
        Ok(true) => {
            println!("Save file detected");

            // let file_path = "saves/player.json";
            // let loaded_save = serde_json::from_reader(file_path).expect("Error");
            // println!("This is the data {:?}", loaded_save)
        }

        Ok(false) => {
            println!("User has no progress");
            println!("Creating a new character");
            handle_new_character_creation();
        }

        Err(e) => {
            println!("Error checkin for saves: {}", e)
        }
    }

    // let first_boss = generate_boss();

    // println!("This is a new user created! {:?}", first_boss);
    // return;

    let chosen_item = Select::new()
        .with_prompt("Menu")
        .default(0)
        .items(&game_menu_options)
        .interact()
        .unwrap();

    // let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];
    match game_menu_options[chosen_item] {
        "Fight" => println!("Fight now"),
        "Weapon Store" => println!("Welcome Stranger"),
        "Quit Game" => println!("Exiting Game"),
        &_ => todo!(),
    }

    // println!("You chose {}", game_menu_options[chosen_item])
}

// CHARACTER FUNCTIONS
// Prompts name, create struct, save character and send in chat confirmation
fn handle_new_character_creation() {
    let name: String = Input::new()
        .with_prompt("Insert your character name")
        .interact_text()
        .unwrap();

    let new_chatacter = create_character(name);

    save_new_character(&new_chatacter);

    println!("Character created! {:?}", new_chatacter)
}

fn create_character(char_name: String) -> Player {
    Player::new_player(char_name)
}

fn save_new_character(new_character: &Player) {
    let serialized_char =
        serde_json::to_string_pretty(&new_character).expect("Failed to serialize player data.");

    fs::write("saves/player.json", serialized_char).expect("Failed to save player JSON to file.");
}

// BOSS FUNCTIONS
fn generate_boss() -> Boss {
    Boss::create_boss(String::from("Jack"), 500, 10, 2, 100)
}
