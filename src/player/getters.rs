use std::fs;

use crate::models::player_models::Player;

pub fn get_player_stats() -> Player {
    let player_stats = fs::read_to_string("saves/player.json")
        .expect("Error on 'get_player_stats': failed to access player data.");

    serde_json::from_str(&player_stats)
        .expect("Error on 'get_player_stats': failed to return player data.")
}
