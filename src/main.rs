mod models;

pub use crate::models::player::Player;

fn main() {
    let new_user = create_character();

    println!(
        "This is a new user created! {}, {}, {}",
        new_user.name, new_user.hp, new_user.defense
    );
}

#[warn(unused)]
fn create_character() -> Player {
    Player::new_player()
}
