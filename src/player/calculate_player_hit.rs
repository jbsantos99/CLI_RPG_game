use crate::utils::{check_crit_hit::check_crit_hit, random_number::random_number};

pub fn calculate_player_hit(
    player_hit_range: (u32, u32),
    player_crit_chance: u32,
    boss_defence_range: (u32, u32),
) -> (u32, bool) {
    let crit_value = check_crit_hit(player_crit_chance);

    let is_critical = crit_value == 2;

    (
        random_number(player_hit_range) * crit_value - random_number(boss_defence_range),
        is_critical,
    )
}
