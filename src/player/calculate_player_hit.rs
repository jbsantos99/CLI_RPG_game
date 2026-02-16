use crate::utils::{check_crit_hit::check_crit_hit, random_number::random_number};

pub fn calculate_player_hit(
    player_hit_range: (u32, u32),
    player_crit_chance: u32,
    boss_defence_range: (u32, u32),
) -> (u32, bool) {
    let crit_value = check_crit_hit(player_crit_chance);

    let boss_defence = random_number(boss_defence_range);
    let player_hit = random_number(player_hit_range);

    let is_critical = crit_value == 2;

    if boss_defence >= player_hit * crit_value {
        return (0, is_critical);
    } else {
        return (player_hit * crit_value - boss_defence, is_critical);
    }
}
