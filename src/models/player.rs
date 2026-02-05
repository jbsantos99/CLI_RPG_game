use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub coins_balance: u32,
}

impl Player {
    pub fn new_player(input_name: String) -> Player {
        Player {
            name: input_name,
            hp: 100,
            attack: 10,
            defense: 1,
            coins_balance: 0,
        }
    }
}
