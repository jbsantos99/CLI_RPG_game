use std::cell::Cell;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Boss {
    pub name: String,
    pub hp: u32,
    pub attack_range: (u32, u32),
    pub defence_range: (u32, u32),
    pub crit_chance: u32,
    pub reward: u32,
    pub is_defeated: Cell<bool>,
}

impl Boss {
    pub fn create_boss(
        name: String,
        hp: u32,
        attack_range: (u32, u32),
        defence_range: (u32, u32),
        crit_chance: u32,
        reward: u32,
    ) -> Boss {
        Boss {
            name,
            hp,
            attack_range,
            defence_range,
            crit_chance,
            reward,
            is_defeated: Cell::new(false),
        }
    }

    pub fn defeated(&self) {
        self.is_defeated.set(true);
    }

    pub fn get_reward(&self) -> u32 {
        if self.is_defeated.get() {
            return self.reward / 2;
        }

        self.reward
    }
}

pub const BOSS_NAMES: [&str; 11] = [
    "Sorretto",
    "Chains",
    "Fumes",
    "Strings",
    "Riot",
    "Sgt. Jacks",
    "Cpl. Dorsman",
    "Apollo",
    "Erskine",
    "Bullet King",
    "|-- Back to Menu --|",
];
