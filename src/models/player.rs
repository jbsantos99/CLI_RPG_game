use std::{cell::Cell, fs};

use serde::{Deserialize, Serialize};

use crate::models::bosses::Boss;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hp: Cell<u32>,
    pub attack_range: Cell<(u32, u32)>,
    pub defense_range: Cell<(u32, u32)>,
    pub crit_chance: Cell<u32>,
    pub coins_balance: Cell<u32>,
}

// impl fmt::Display for Player {
//     fn fmt(&self, formatter: fmt::Formatter) -> fmt::Result {
//         write!(formatter, "{} {}", self.)
//     }
// }

impl Player {
    pub fn new_player(input_name: String) -> Player {
        Player {
            name: input_name,
            hp: Cell::new(100),
            attack_range: Cell::new((15, 20)),
            defense_range: Cell::new((1, 4)),
            crit_chance: Cell::new(10),
            coins_balance: Cell::new(0),
        }
    }

    pub fn get_info(&self) -> &Player {
        return self;
    }

    pub fn save(&self) {
        let parsed_player_data =
            serde_json::to_string_pretty(&self).expect("Failed to parse player data to save");

        fs::write("saves/player.json", parsed_player_data).expect("Failed to save player data");
    }

    pub fn reset_save(&self) {
        fs::remove_file("saves/player.json").expect("failed at 'reset_save' for Player");
    }

    pub fn incr_coins(&self, amount: u32) {
        self.coins_balance.set(self.coins_balance.get() + amount);
    }

    pub fn decr_coins(&self, amount: u32) {
        if amount > self.coins_balance.get() {
            return println!("Not enough cash, stranger!");
        }

        self.coins_balance.set(self.coins_balance.get() - amount)
    }

    pub fn collect_boss_rewards(&self, boss: &Boss) {
        self.incr_coins(boss.get_reward());
        self.save();
    }
}
