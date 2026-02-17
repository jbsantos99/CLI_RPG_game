use crate::models::{bosses::Boss, player::Player};

pub fn compare_player_boss(player: &Player, boss: &Boss) {
    println!("");

    println!(
        "{}  -  HP {} | Attack {}-{} | Deffence {}-{} | Crit Chance {}",
        player.name,
        player.hp.get(),
        player.attack_range.get().0,
        player.attack_range.get().1,
        player.defense_range.get().0,
        player.defense_range.get().1,
        player.crit_chance.get(),
    );

    println!(
        "{}  -  HP {} | Attack {}-{} | Deffence {}-{} | Crit Chance {}",
        boss.name,
        boss.hp,
        boss.attack_range.0,
        boss.attack_range.1,
        boss.defence_range.0,
        boss.defence_range.1,
        boss.crit_chance
    );

    println!("");
}
