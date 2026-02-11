use dialoguer::Select;

use crate::{
    actions::fight::fight_boss,
    menus::boss_menu::launch_boss_menu,
    models::{bosses::Boss, player_models::Player},
    player::getters::get_player_stats,
    utils::clear_terminal::clear_terminal,
};

pub fn launch_boss_fight_menu(selected_boss: &Boss, boss_index: usize) {
    clear_terminal();
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
