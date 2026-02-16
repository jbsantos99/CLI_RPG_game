use crate::utils::{check_crit_hit::check_crit_hit, random_number::random_number};

pub fn calculate_boss_hit(
    boss_hit_range: (u32, u32),
    boss_crit_chance: u32,
    player_defence_range: (u32, u32),
) -> (u32, bool) {
    let crit_value = check_crit_hit(boss_crit_chance);

    let player_defence = random_number(player_defence_range);
    let boss_hit = random_number(boss_hit_range);

    let is_critical = crit_value == 2;

    if player_defence >= boss_hit * crit_value {
        return (0, is_critical);
    } else {
        return (boss_hit * crit_value - player_defence, is_critical);
    }
}
