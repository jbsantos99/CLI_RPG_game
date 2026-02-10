use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Boss {
    pub name: String,
    pub hp: i32,
    pub attack: u32,
    pub defence: u32,
    pub reward: u32,
    pub is_defeated: bool,
}

impl Boss {
    pub fn create_boss(
        name: String,
        hp: i32,
        attack: u32,
        defence: u32,
        reward: u32,
        is_defeated: bool,
    ) -> Boss {
        Boss {
            name,
            hp,
            attack,
            defence,
            reward,
            is_defeated,
        }
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
