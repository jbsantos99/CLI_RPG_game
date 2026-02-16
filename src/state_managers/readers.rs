use std::fs;

use crate::models::bosses::Boss;

pub fn get_boss_list() -> Vec<Boss> {
    let raw_data = fs::read_to_string("saves/bosses.json")
        .expect("Error at 'get_boss_list': failed to read file.");

    let boss_list: Vec<Boss> =
        serde_json::from_str(&raw_data).expect("Error at 'get_boss_list': failed to parse file.");

    boss_list
}

pub fn get_boss_by_id(boss_id: usize) -> Boss {
    let raw_data = fs::read_to_string("saves/bosses.json")
        .expect("Error at 'get_boss_list': failed to read file.");

    let boss_list: Vec<Boss> =
        serde_json::from_str(&raw_data).expect("Error at 'get_boss_list': failed to parse file.");

    boss_list[boss_id].clone()
}
