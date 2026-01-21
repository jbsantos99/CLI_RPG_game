mod models;
use crate::models::bosses::Boss;
use crate::models::player::Player;
use serde::Serialize;

use ::std::fs;
use std::fs::File;

use dialoguer::Input;
use dialoguer::Select;
fn main() {
    let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];

    let name: String = Input::new()
        .with_prompt("Insert your character name")
        .interact_text()
        .unwrap();

    let new_chatacter = create_character(name);

    let serialized_char =
        serde_json::to_string_pretty(&new_chatacter).expect("Failed to serialize player data.");

    println!("this is our json char {}", serialized_char);

    fs::write("player.json", serialized_char).expect("Failed to save player JSON to file.");

    let first_boss = generate_boss();

    println!("This is a new user created! {:?}", first_boss);
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

fn create_character(char_name: String) -> Player {
    Player::new_player(char_name)
}

fn generate_boss() -> Boss {
    Boss::create_boss(String::from("Jack"), 500, 10, 2, 100)
}
