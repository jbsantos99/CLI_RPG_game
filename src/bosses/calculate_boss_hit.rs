use crate::utils::{check_crit_hit::check_crit_hit, random_number::random_number};

pub fn calculate_boss_hit(
    boss_hit_range: (u32, u32),
    boss_crit_chance: u32,
    player_defence_range: (u32, u32),
) -> u32 {
    random_number(boss_hit_range) * check_crit_hit(boss_crit_chance)
        - random_number(player_defence_range)
}
