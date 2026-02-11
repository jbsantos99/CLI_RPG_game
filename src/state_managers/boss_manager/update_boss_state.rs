use std::fs;

use crate::{models::bosses::Boss, state_managers::save_boss_array::save_generated_boss_array};

pub fn update_boss_state(boss_index: usize) {
    // no need to check boss save files, since they are required to get to this point
    let bosses_file =
        fs::read_to_string("saves/bosses.json").expect("Failed to load boss file for saving");

    let mut bosses_json: Vec<Boss> = serde_json::from_str(&bosses_file).expect("a");

    println!("update boss {:#?}", bosses_json[boss_index].name);

    bosses_json[boss_index].is_defeated = true;

    save_generated_boss_array(&bosses_json);
}
