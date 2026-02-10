mod actions;
mod models;
mod player;
mod utils;

use crate::actions::fight::fight;
use crate::actions::fight::FightResult;
use crate::models::bosses::Boss;

use crate::models::player_models::Player;
use crate::utils::random_number::random_number;
use ::std::fs;
use ::std::io::Result;

// GETTERS
use crate::player::getters::get_player_stats;

use dialoguer::Input;
use dialoguer::Select;
//
// global variables
use crate::models::bosses::BOSS_NAMES;

fn check_saves() -> Result<bool> {
    let file_path = "saves/player.json";

    match fs::exists(file_path) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            eprintln!("Error checking for save files: {}", e);
            Err(e)
        }
    }
}

fn check_boss_saves() -> Result<bool> {
    let file_path = "saves/bosses.json";

    match fs::exists(file_path) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            eprintln!("Error checking for bosses save files: {}", e);
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
        "Fight" => launch_boss_menu(),
        "Weapon Store" => println!("Welcome Stranger"),
        "Quit Game" => println!("Exiting Game"),
        &_ => todo!(),
    }
}

fn launch_boss_menu() {
    match check_boss_saves() {
        Ok(true) => {}
        Ok(false) => {
            println!("Bosses save files not detected!");
            generate_bosses();
        }

        Err(err) => {
            println!("Error checking bosses save files {}", err)
        }
    }

    let boss_menu_options = BOSS_NAMES;

    let chosen_item = Select::new()
        .with_prompt("Boss List")
        .default(0)
        .items(&boss_menu_options)
        .interact()
        .unwrap();

    println!("This is the selected boss {:#?}", BOSS_NAMES[chosen_item]);

    let bosses_file =
        fs::read_to_string("saves/bosses.json").expect("failed parsing json save file to string.");

    let target_file: Vec<Boss> =
        serde_json::from_str(&bosses_file).expect("failed reading string save file content");

    let selected_boss = &target_file[chosen_item];
    launch_boss_fight_menu(selected_boss, chosen_item);
}

fn launch_boss_fight_menu(selected_boss: &Boss, boss_index: usize) {
    let player_stats: Player = get_player_stats();

    println!("Boss Stats: {:#?}", selected_boss);
    println!("Your Stats: {:#?}", player_stats);

    // println!("Your Stats: {:#?}", player)
    let boss_actions_menu = vec!["Fight", "Go Back"];

    let chosen_item = Select::new()
        .with_prompt("Take action")
        .default(0)
        .items(&boss_actions_menu)
        .interact()
        .unwrap();

    let copy_selected_boss = selected_boss.clone();

    match boss_actions_menu[chosen_item] {
        "Fight" => fight_boss(&copy_selected_boss, boss_index),
        "Go Back" => launch_boss_menu(),
        &_ => todo!(),
    }
}

fn fight_boss(boss_data: &Boss, boss_index: usize) {
    let player_stats: Player = get_player_stats();

    match fight(&player_stats, boss_data) {
        FightResult::Won => println!("you won!"),
        FightResult::Lost => println!("you lost!"),
    }

    if !boss_data.is_defeated {
        update_boss_state(boss_index);
    }

    launch_boss_menu()
}

fn update_boss_state(boss_index: usize) {
    // no need to check boss save files, since they are required to get to this point
    let bosses_file =
        fs::read_to_string("saves/bosses.json").expect("Failed to load boss file for saving");

    let mut bosses_json: Vec<Boss> = serde_json::from_str(&bosses_file).expect("a");

    println!("update boss {:#?}", bosses_json[boss_index].name);

    bosses_json[boss_index].is_defeated = true;

    save_generated_boss_array(&bosses_json);
}

fn main() {
    match check_saves() {
        Ok(true) => {
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
            (random_number(40, 60) * base_mult).try_into().unwrap(),
            random_number(4, 6) * base_mult,
            random_number(1, 3) * base_mult,
            random_number(40, 80) * base_mult,
            false,
        );

        new_boss_array.push(raw_boss);
    }

    println!("Boss Generating Finished!");
    println!("This is the boss array {:#?}", new_boss_array);

    println!("Saving state...");
    save_generated_boss_array(&new_boss_array);
}

fn save_generated_boss_array(boss_array: &Vec<Boss>) {
    let serialized_bosses =
        serde_json::to_string_pretty(boss_array).expect("Couldn't serialize new boss array.");

    fs::write("saves/bosses.json", serialized_bosses).expect("Couldn't sava serialized bosses.");
    println!("Boss state updated!")
}
