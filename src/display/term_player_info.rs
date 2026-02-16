use crate::player::getters::get_player_stats;

pub fn term_player_info() {
    let player = get_player_stats();

    println!(
        "HP {}pts   |   Attack {}-{}   |   Deffence {}-{}      |     Crit. Chance {}%     |     Gold ${}",
        player.hp.get(),
        player.attack_range.0,
        player.attack_range.1,
        player.defense_range.0,
        player.defense_range.1,
        player.crit_chance,
        player.coins_balance.get(),
    );

    println!()
}
