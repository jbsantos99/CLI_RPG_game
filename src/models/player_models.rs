use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub attack_range: (u32, u32),
    pub defense_range: (u32, u32),
    pub crit_chance: u32,
    pub coins_balance: u32,
}

impl Player {
    pub fn new_player(input_name: String) -> Player {
        Player {
            name: input_name,
            hp: 100,
            attack_range: (10, 20),
            defense_range: (1, 4),
            crit_chance: 10,
            coins_balance: 0,
        }
    }
}
