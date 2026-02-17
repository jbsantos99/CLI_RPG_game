use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    display::{game_commands::display_game_commands, term_player_info::term_player_info},
    menus::main_menu::launch_main_menu,
    models::{merchant::Merchant, player::Player},
    player::getters::get_player_stats,
    utils::clear_terminal::clear_terminal,
};

pub fn launch_merchant_menu() {
    clear_terminal();
    term_player_info();
    display_game_commands();

    let merchant_menu_options = Merchant::get_items();

    let player = get_player_stats();

    let chosen_item = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What are you buying?")
        .default(0)
        .items(&merchant_menu_options)
        .interact_opt()
        .unwrap();

    term_player_info();

    match chosen_item {
        Some(item_id) => {
            Merchant::get_item_by_id(item_id).purchase(Player::get_info(&player));
        }
        None => launch_main_menu(),
    }
}
