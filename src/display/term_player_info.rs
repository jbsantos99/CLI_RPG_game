use crate::player::getters::get_player_stats;

pub fn term_player_info() {
    let player = get_player_stats();

    println!(
        "HP {}pts   |   Attack {}-{}   |   Deffence {}-{}      |     Crit. Chance {}%     |     Gold ${}",
        player.hp.get(),
        player.attack_range.get().0,
        player.attack_range.get().1,
        player.defense_range.get().0,
        player.defense_range.get().1,
        player.crit_chance.get(),
        player.coins_balance.get(),
    );

    println!()
}
