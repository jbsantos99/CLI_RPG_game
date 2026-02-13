use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::{
    // menus::boss_menu::launch_boss_menu, models::bosses::Boss, player::getters::get_player_stats,
    bosses::calculate_boss_hit::calculate_boss_hit,
    menus::boss_menu::launch_boss_menu,
    models::{bosses::Boss, player_models::Player},
    player::{calculate_player_hit::calculate_player_hit, getters::get_player_stats},
    progress_bars::update_progbar_msg::update_progbar_msg,
    state_managers::boss_manager::update_boss_state::update_boss_state,
    utils::{clear_terminal::clear_terminal, delay_in_ms::delay_in_ms, line_spacer::line_spacer},
};

pub fn handle_close_progbar(prog_bars: &[&ProgressBar]) {
    for val in prog_bars {
        val.finish()
    }
}

pub fn handle_battle_won(boss: &Boss, boss_index: usize) {
    update_boss_state(boss_index);
}

pub fn collect_boss_rewards(player: &Player, boss: &Boss) {
    delay_in_ms();
    println!("before {}", player.coins_balance.get());

    player.incr_coins(boss.get_reward());
    player.save();

    println!("after {}", player.coins_balance.get());

    println!("after {:#?}", player);
    delay_in_ms();
    delay_in_ms();
    delay_in_ms();
    delay_in_ms();
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
            boss_hp_bar.dec(hit_given as u64);

            update_progbar_msg(&boss_hp_bar, hit_given, is_given_crit);

            delay_in_ms();

            // we can take it
            if hit_taken < p_hp {
                p_hp -= hit_taken;
                player_hp_bar.dec(hit_taken as u64);

                update_progbar_msg(&player_hp_bar, hit_taken, is_taken_crit);

                delay_in_ms();
            } else {
                p_hp = 0;

                player_hp_bar.set_position(0);
                player_hp_bar.finish();

                handle_close_progbar(&[&spacer_bar_top, &spacer_bar_midle, &spacer_bar_bottom]);

                clear_terminal();
                println!("Player defeated!")
            }
        } else {
            b_hp = 0;

            boss_hp_bar.set_position(0);
            boss_hp_bar.finish();

            handle_battle_won(boss_stats, boss_index);

            handle_close_progbar(&[&spacer_bar_top, &spacer_bar_midle, &spacer_bar_bottom]);

            clear_terminal();

            collect_boss_rewards(&player_stats, &boss_stats);
            println!("Boss defeated!");
        }
    }

    delay_in_ms();

    clear_terminal();
    launch_boss_menu();
}
