use std::{fs, io::Result};

pub fn check_player_save_files() -> Result<bool> {
    let file_path = "saves/player.json";

    match fs::exists(file_path) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            eprintln!("Error checking for player save files: {}", e);
            Err(e)
        }
    }
}

pub fn check_boss_saves_files() -> Result<bool> {
    let file_path = "saves/bosses.json";

    match fs::exists(file_path) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            eprintln!("Error checking for bosses save files: {}", e);
            Err(e)
        }
    }
}
