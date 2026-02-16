use std::{cell::Cell, fmt};

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

impl fmt::Display for Boss {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{} \nHP: {}   |   Attack: {}-{}   |   deffence: {}-{}   |   reward: {} \n",
            self.name,
            self.hp,
            self.attack_range.0,
            self.attack_range.1,
            self.defence_range.0,
            self.defence_range.1,
            self.reward,
        )
    }
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

pub const BOSS_NAMES: [&str; 10] = [
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
];
