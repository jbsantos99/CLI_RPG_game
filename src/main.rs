mod actions;
mod bosses;
mod display;
mod menus;
mod models;
mod player;
mod progress_bars;
mod state_managers;
mod utils;

// Menus
use crate::menus::main_menu::launch_main_menu;

use crate::progress_bars::loading_bar::loading_bar;
// State management
use crate::state_managers::character_creation::handle_new_character_creation;
use crate::state_managers::check_for_save_files::check_player_save_files;
use crate::utils::clear_terminal::clear_terminal;

fn main() {
    clear_terminal();

    println!("");
    println!("Welcome to Rust CLI Game!");
    println!("");

    loading_bar(2500, String::from("Checking save files"));

    if check_player_save_files() {
        launch_main_menu();
    } else {
        println!("User has no progress.");
        println!("Creating a new character!");
        handle_new_character_creation();
    }
}
