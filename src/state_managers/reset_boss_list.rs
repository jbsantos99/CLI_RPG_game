use std::fs;

use crate::bosses::generate_boss_list::generate_bosses;

pub fn reset_boss_list() {
    fs::remove_file("saves/bosses.json");
    generate_bosses();
}
