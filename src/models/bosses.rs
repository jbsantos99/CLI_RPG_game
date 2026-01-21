#[derive(Debug)]
pub struct Boss {
    name: String,
    hp: u32,
    attack: u32,
    defence: u32,
    reward: u32,
}

impl Boss {
    pub fn create_boss(name: String, hp: u32, attack: u32, defence: u32, reward: u32) -> Boss {
        Boss {
            name,
            hp,
            attack,
            defence,
            reward,
        }
    }
}
