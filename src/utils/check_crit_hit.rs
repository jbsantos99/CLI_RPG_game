use crate::utils::random_number::random_number;

pub fn check_crit_hit(crit_hit_chance: u32) -> u32 {
    let random_numb = random_number((0, 101));

    if random_numb > crit_hit_chance {
        return 1;
    }

    2
}
