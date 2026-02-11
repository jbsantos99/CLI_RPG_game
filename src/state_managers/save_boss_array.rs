use std::fs;

use crate::models::bosses::Boss;

pub fn save_generated_boss_array(boss_array: &Vec<Boss>) {
    let serialized_bosses =
        serde_json::to_string_pretty(boss_array).expect("Couldn't serialize new boss array.");

    fs::write("saves/bosses.json", serialized_bosses).expect("Couldn't sava serialized bosses.");
    println!("Boss state updated!")
}
