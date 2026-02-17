use dialoguer::Select;

use crate::{
    display::{game_commands::display_game_commands, term_player_info::term_player_info},
    main,
    menus::main_menu::launch_main_menu,
    player::getters::get_player_stats,
    state_managers::reset_boss_list::reset_boss_list,
    utils::delay_in_ms::delay_in_ms,
};

pub fn launch_saves_menu() {
    term_player_info();
    display_game_commands();

    let game_menu_options = vec!["Reset Player", "Reset Bosses"];

    let chosen_item = Select::new()
        .with_prompt("Save File Options")
        .default(0)
        .items(&game_menu_options)
        .interact_opt()
        .unwrap();

    match chosen_item {
        Some(option) => {
            match game_menu_options[option] {
                "Reset Player" => {
                    let player = get_player_stats();
                    player.reset_save();

                    println!("Player data reset. Back to main menu.");
                    delay_in_ms(1500);
                    main();
                }

                "Reset Bosses" => {
                    reset_boss_list();
                    println!("Boss list reset. Back to main menu.");
                    delay_in_ms(1500);

                    main()
                }

                &_ => todo!(),
            }
            println!("{}", option)
        }
        None => launch_main_menu(),
    }
}
