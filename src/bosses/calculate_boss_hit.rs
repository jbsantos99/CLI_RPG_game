use crate::utils::{check_crit_hit::check_crit_hit, random_number::random_number};

pub fn calculate_boss_hit(
    boss_hit_range: (u32, u32),
    boss_crit_chance: u32,
    player_defence_range: (u32, u32),
) -> (u32, bool) {
    let crit_value = check_crit_hit(boss_crit_chance);

    let is_critical = crit_value == 2;

    (
        random_number(boss_hit_range) * crit_value - random_number(player_defence_range),
        is_critical,
    )
}
