use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    bosses::generate_boss_list::generate_bosses,
    display::{game_commands::display_game_commands, term_player_info::term_player_info},
    menus::{boss_fight_menu::launch_boss_fight_menu, main_menu::launch_main_menu},
    models::bosses::Boss,
    state_managers::{
        check_for_save_files::check_boss_saves_files,
        readers::{get_boss_by_id, get_boss_list},
    },
    utils::clear_terminal::clear_terminal,
};

pub fn launch_boss_menu() {
    clear_terminal();
    term_player_info();
    display_game_commands();

    if !check_boss_saves_files() {
        generate_bosses();
    }

    let boss_list: Vec<Boss> = get_boss_list();

    let chosen_item = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Boss List")
        .default(0)
        .items(boss_list)
        .max_length(5)
        .interact_opt()
        .unwrap();

    match chosen_item {
        Some(boss_index) => launch_boss_fight_menu(&get_boss_by_id(boss_index), boss_index),
        None => {
            launch_main_menu();
        }
    }
}
