use crate::utils::{check_crit_hit::check_crit_hit, random_number::random_number};

pub fn calculate_player_hit(
    player_hit_range: (u32, u32),
    player_crit_chance: u32,
    boss_defence_range: (u32, u32),
) -> u32 {
    random_number(player_hit_range) * check_crit_hit(player_crit_chance)
        - random_number(boss_defence_range)
}
