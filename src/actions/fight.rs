use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::{
    bosses::calculate_boss_hit::calculate_boss_hit,
    menus::boss_menu::launch_boss_menu,
    models::bosses::Boss,
    player::{calculate_player_hit::calculate_player_hit, getters::get_player_stats},
    progress_bars::{hp_bar::update_hp_on_hit, update_progbar_msg::close_progbars},
    state_managers::boss_manager::update_boss_state::update_boss_state,
    utils::{clear_terminal::clear_terminal, delay_in_ms::delay_in_ms, line_spacer::line_spacer},
};

pub fn handle_battle_won(boss_index: usize) {
    update_boss_state(boss_index);
}

pub fn fight_boss(boss_stats: &Boss, boss_index: usize) {
    clear_terminal();

    println!("Fighting against {}", boss_stats.name);
    line_spacer(1);

    let player_stats = get_player_stats();

    let mut p_battle_hp = player_stats.hp.get();
    let mut b_hp = boss_stats.hp;

    let multi_bar = MultiProgress::new();

    let spacer_bar_top = multi_bar.add(ProgressBar::new_spinner());
    let boss_hp_bar = multi_bar.add(ProgressBar::new(b_hp as u64));
    let spacer_bar_midle = multi_bar.add(ProgressBar::new_spinner());
    let player_hp_bar = multi_bar.add(ProgressBar::new(p_battle_hp as u64));
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

    player_hp_bar.set_position(p_battle_hp as u64);
    boss_hp_bar.set_position(b_hp as u64);

    while p_battle_hp > 0 && b_hp > 0 {
        delay_in_ms(1000);

        let (hit_given, is_given_crit) = calculate_player_hit(
            player_stats.attack_range,
            player_stats.crit_chance,
            boss_stats.defence_range,
        );

        let (hit_taken, is_taken_crit) = calculate_boss_hit(
            boss_stats.attack_range,
            boss_stats.crit_chance,
            player_stats.defense_range,
        );

        // will this hit kill the boss?
        if hit_given < b_hp {
            b_hp -= hit_given;

            update_hp_on_hit(&boss_hp_bar, hit_given, is_given_crit);

            delay_in_ms(1000);

            // we can take it
            if hit_taken < p_battle_hp {
                p_battle_hp -= hit_taken;
                update_hp_on_hit(&player_hp_bar, hit_taken, is_taken_crit);
            } else {
                p_battle_hp = 0;
                player_hp_bar.set_position(0);

                close_progbars(&[
                    &player_hp_bar,
                    &spacer_bar_top,
                    &spacer_bar_midle,
                    &spacer_bar_bottom,
                ]);

                clear_terminal();
                println!("Player defeated!")
            }
        } else {
            b_hp = 0;

            boss_hp_bar.set_position(0);

            handle_battle_won(boss_index);

            close_progbars(&[
                &boss_hp_bar,
                &spacer_bar_top,
                &spacer_bar_midle,
                &spacer_bar_bottom,
            ]);

            clear_terminal();

            player_stats.collect_boss_rewards(&boss_stats);
            println!("Boss defeated!");
        }
    }

    delay_in_ms(1000);

    clear_terminal();
    launch_boss_menu();
}
