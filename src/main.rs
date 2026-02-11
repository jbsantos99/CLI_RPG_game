mod actions;
mod bosses;
mod menus;
mod models;
mod player;
mod state_managers;
mod utils;

// Menus
use crate::menus::main_menu::launch_main_menu;

// State management
use crate::state_managers::character_creation::handle_new_character_creation;
use crate::state_managers::check_for_save_files::check_player_save_files;

fn main() {
    match check_player_save_files() {
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
