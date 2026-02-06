mod models;
mod utils;

use crate::models::bosses::Boss;
use crate::models::player::Player;

use crate::utils::random_number::random_number;

use ::std::fs;

use dialoguer::Input;
use dialoguer::Select;

// global variables
use crate::models::bosses::BOSS_NAMES;

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

fn launch_main_menu() {
    let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];

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
}

fn launch_boss_menu() {
    let boss_menu_options = vec!["", "Weapon Store", "Back"];
    //
    // let chosen_item = Select::new()
    //     .with_prompt("Menu")
    //     .default(0)
    //     .items(&game_menu_options)
    //     .interact()
    //     .unwrap();
    //
    // // let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];
    // match game_menu_options[chosen_item] {
    //     "Fight" => println!("Fight now"),
    //     "Weapon Store" => println!("Welcome Stranger"),
    //     "Quit Game" => println!("Exiting Game"),
    //     &_ => todo!(),
    // }
}

fn main() {
    generate_bosses();
    return;

    match check_saves() {
        Ok(true) => {
            println!("Save file detected");
            let save_file = fs::read_to_string("saves/player.json")
                .expect("failed parsing json save file to string.");

            let user_file: Player =
                serde_json::from_str(&save_file).expect("failed reading string save file content");

            println!("this is the user file {:?}", user_file);

            launch_main_menu();
        }

        Ok(false) => {
            println!("User has no progress.");
            println!("Creating a new character!");
            handle_new_character_creation();
        }

        Err(e) => {
            println!("Error checkin for saves: {}", e)
        }
    };

    // let first_boss = generate_boss();

    // println!("This is a new user created! {:?}", first_boss);
    // return;

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

    println!("Character created! {:?}", new_chatacter);

    launch_main_menu();
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
fn generate_bosses() {
    let mut new_boss_array: Vec<Boss> = Vec::new();

    for (index, &item) in BOSS_NAMES.iter().enumerate() {
        println!("Generating {:?}", &item);

        let base_mult = index as u32 + 1;

        let raw_boss = Boss::create_boss(
            String::from(item),
            random_number(400, 600) * base_mult,
            random_number(8, 12) * base_mult,
            random_number(1, 3) * base_mult,
            random_number(80, 120) * base_mult,
            false,
        );

        new_boss_array.push(raw_boss);
    }

    println!("Boss Generating Finished!");
    println!("This is the boss array {:#?}", new_boss_array);

    println!("Saving state...");
    save_freshly_generated_boss_array(&new_boss_array);
}

fn save_freshly_generated_boss_array(boss_array: &Vec<Boss>) {
    let serialized_bosses =
        serde_json::to_string_pretty(boss_array).expect("Couldn't serialize new boss array.");

    fs::write("saves/bosses.json", serialized_bosses).expect("Couldn't sava serialized bosses.");
}
