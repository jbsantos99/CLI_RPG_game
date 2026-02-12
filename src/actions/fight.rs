use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde_json::to_string;

use crate::{
    // menus::boss_menu::launch_boss_menu, models::bosses::Boss, player::getters::get_player_stats,
    bosses::calculate_boss_hit::calculate_boss_hit,
    menus::boss_menu::launch_boss_menu,
    models::bosses::Boss,
    player::{calculate_player_hit::calculate_player_hit, getters::get_player_stats},
    state_managers::boss_manager::update_boss_state::update_boss_state,
    utils::{clear_terminal::clear_terminal, delay_in_ms::delay_in_ms, line_spacer::line_spacer},
};
pub fn update_progbar_msg(progbar: &ProgressBar, msg: String) {
    progbar.set_message(msg);
}

pub fn fight_boss(boss_stats: &Boss, boss_index: usize) {
    clear_terminal();

    println!("Fighting against {}", boss_stats.name);
    line_spacer(1);

    let player_stats = get_player_stats();

    let mut p_hp = player_stats.hp;
    let mut b_hp = boss_stats.hp;

    let multi_bar = MultiProgress::new();

    let spacer_bar_top = multi_bar.add(ProgressBar::new_spinner());
    let boss_hp_bar = multi_bar.add(ProgressBar::new(b_hp as u64));
    let spacer_bar_midle = multi_bar.add(ProgressBar::new_spinner());
    let player_hp_bar = multi_bar.add(ProgressBar::new(p_hp as u64));
    let spacer_bar_bottom = multi_bar.add(ProgressBar::new_spinner());

    spacer_bar_top.set_position(0);
    spacer_bar_top.enable_steady_tick(Duration::from_millis(200));

    player_hp_bar.set_message(player_stats.name.clone());

    spacer_bar_midle.set_position(0);
    spacer_bar_midle.enable_steady_tick(Duration::from_millis(100));

    boss_hp_bar.set_message(boss_stats.name.clone());

    spacer_bar_bottom.set_position(0);
    spacer_bar_bottom.enable_steady_tick(Duration::from_millis(300));

    player_hp_bar
        .set_style(ProgressStyle::with_template("{bar:80.green/grey} {pos}/{len} {msg}").unwrap());

    boss_hp_bar
        .set_style(ProgressStyle::with_template("{bar:80.yellow/grey} {pos}/{len} {msg}").unwrap());

    player_hp_bar.set_position(p_hp as u64);
    boss_hp_bar.set_position(b_hp as u64);

    while p_hp > 0 && b_hp > 0 {
        delay_in_ms();

        let hit_given = calculate_player_hit(
            player_stats.attack_range,
            player_stats.crit_chance,
            boss_stats.defence_range,
        );

        let hit_taken = calculate_boss_hit(
            boss_stats.attack_range,
            boss_stats.crit_chance,
            player_stats.defense_range,
        );

        // will this hit kill the boss?
        if hit_given < b_hp {
            b_hp -= hit_given;
            boss_hp_bar.dec(hit_given as u64);

            update_progbar_msg(
                &boss_hp_bar,
                [
                    boss_stats.name.clone(),
                    String::from("  -"),
                    hit_given.to_string(),
                ]
                .join(""),
            );

            delay_in_ms();
            update_progbar_msg(&boss_hp_bar, boss_stats.name.clone());
            delay_in_ms();
            // we can take it
            if hit_taken < p_hp {
                p_hp -= hit_taken;
                player_hp_bar.dec(hit_taken as u64);

                update_progbar_msg(
                    &player_hp_bar,
                    [
                        player_stats.name.clone(),
                        String::from("  -"),
                        hit_taken.to_string(),
                    ]
                    .join(""),
                );

                delay_in_ms();
                update_progbar_msg(&player_hp_bar, player_stats.name.clone());
            } else {
                p_hp = 0;

                player_hp_bar.set_position(0);
                player_hp_bar.finish();

                spacer_bar_bottom.finish();
                spacer_bar_midle.finish();
                spacer_bar_top.finish();

                clear_terminal();
                println!("Player defeated!")
            }
        } else {
            b_hp = 0;

            boss_hp_bar.set_position(0);
            boss_hp_bar.finish();

            spacer_bar_bottom.finish();
            spacer_bar_midle.finish();
            spacer_bar_top.finish();

            update_boss_state(boss_index);
            clear_terminal();

            println!("Boss defeated!");
        }
    }

    delay_in_ms();
    clear_terminal();
    launch_boss_menu();
}
