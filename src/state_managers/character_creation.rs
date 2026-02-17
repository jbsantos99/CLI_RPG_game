use std::fs;

use dialoguer::Input;

use crate::{launch_main_menu, models::player::Player};

pub fn handle_new_character_creation() {
    let name: String = Input::new()
        .with_prompt("Insert your character name")
        .interact_text()
        .unwrap();

    let new_chatacter = create_character(name);

    save_new_character(&new_chatacter);

    println!("Character created! {:?}", new_chatacter);

    launch_main_menu();
}

pub fn create_character(char_name: String) -> Player {
    Player::new_player(char_name)
}

pub fn save_new_character(new_character: &Player) {
    let serialized_char =
        serde_json::to_string_pretty(&new_character).expect("Failed to serialize player data.");

    fs::write("saves/player.json", serialized_char).expect("Failed to save player JSON to file.");
}
