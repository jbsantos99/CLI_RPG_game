use dialoguer::Select;

use crate::menus::boss_menu::launch_boss_menu;

pub fn launch_main_menu() {
    let game_menu_options = vec!["Fight", "Merchant", "Quit Game"];

    let chosen_item = Select::new()
        .with_prompt("Menu")
        .default(0)
        .items(&game_menu_options)
        .interact()
        .unwrap();

    // let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];
    match game_menu_options[chosen_item] {
        "Fight" => launch_boss_menu(),
        "Merchant" => println!("Welcome Stranger"),
        "Quit Game" => println!("Exiting Game"),
        &_ => todo!(),
    }
}
