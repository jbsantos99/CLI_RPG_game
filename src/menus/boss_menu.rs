use std::fs;

use dialoguer::Select;

use crate::{
    bosses::generate_boss_list::generate_bosses,
    menus::{boss_fight_menu::launch_boss_fight_menu, main_menu::launch_main_menu},
    models::bosses::{Boss, BOSS_NAMES},
    state_managers::check_for_save_files::check_boss_saves_files,
};

pub fn launch_boss_menu() {
    match check_boss_saves_files() {
        Ok(true) => {}
        Ok(false) => {
            println!("Bosses save files not detected!");
            generate_bosses();
        }

        Err(err) => {
            println!("Error checking bosses save files {}", err)
        }
    }

    let chosen_item = Select::new()
        .with_prompt("Boss List")
        .default(0)
        .items(&BOSS_NAMES)
        .interact()
        .unwrap();

    match chosen_item {
        10 => launch_main_menu(),
        _ => {
            println!("This is the selected boss {:#?}", BOSS_NAMES[chosen_item]);

            let bosses_file = fs::read_to_string("saves/bosses.json")
                .expect("failed parsing json save file to string.");

            let target_file: Vec<Boss> = serde_json::from_str(&bosses_file)
                .expect("failed reading string save file content");

            let selected_boss = &target_file[chosen_item];

            launch_boss_fight_menu(selected_boss, chosen_item);
            println!("chosen item{}", chosen_item);
        }
    }
}
