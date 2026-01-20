mod models;

pub use crate::models::player::Player;
use dialoguer::Input;

fn main() {
    let name: String = Input::new()
        .with_prompt("Insert your character name")
        .interact_text()
        .unwrap();

    let new_chatacter = create_character(name);

    println!(
        "This is a new user created! {}, {}, {}",
        new_chatacter.name, new_chatacter.hp, new_chatacter.defense
    );
}

#[warn(unused)]
fn create_character(char_name: String) -> Player {
    Player::new_player(char_name)
}
