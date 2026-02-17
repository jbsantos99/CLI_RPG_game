use dialoguer::Select;

use crate::{
    display::{game_commands::display_game_commands, term_player_info::term_player_info},
    menus::{
        boss_menu::launch_boss_menu, merchant::launch_merchant_menu, saves::launch_saves_menu,
    },
    utils::clear_terminal::clear_terminal,
};

pub fn launch_main_menu() {
    clear_terminal();
    term_player_info();

    let game_menu_options = vec!["Fight", "Merchant", "Saves", "Quit Game"];

    let chosen_item = Select::new()
        .with_prompt("Menu")
        .default(0)
        .items(&game_menu_options)
        .interact()
        .unwrap();

    match game_menu_options[chosen_item] {
        "Fight" => launch_boss_menu(),
        "Merchant" => launch_merchant_menu(),
        "Quit Game" => println!("Exiting Game"),
        "Saves" => launch_saves_menu(),
        &_ => todo!(),
    }
}
