use std::fs;
pub fn check_player_save_files() -> bool {
    let file_path = "saves/player.json";
    fs::exists(file_path).expect("Fail on 'check_player_save_files'")
}

pub fn check_boss_saves_files() -> bool {
    let file_path = "saves/bosses.json";
    fs::exists(file_path).expect("Fail on 'check_boss_saves_files'")
}
