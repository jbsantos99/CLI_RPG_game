mod models;

use crate::models::bosses::Boss;
use crate::models::player::Player;

use dialoguer::Input;
use dialoguer::Select;

fn main() {
    let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];

    let name: String = Input::new()
        .with_prompt("Insert your character name")
        .interact_text()
        .unwrap();

    let new_chatacter = create_character(name);
    let first_boss = generate_boss();

    println!(
        "This is a new user created! {}, {}, {}",
        new_chatacter.name, new_chatacter.hp, new_chatacter.defense
    );

    println!("This is a new user created! {:?}", first_boss);

    let chosen_item = Select::new()
        .with_prompt("Menu")
        .default(0)
        .items(&game_menu_options)
        .interact()
        .unwrap();

    // let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];
    match game_menu_options[chosen_item] {
        "Fight" => println!("Fight now"),
        "Weapon Store" => println!("Welcome Stranger"),
        "Quit Game" => println!("Exiting Game"),
        &_ => todo!(),
    }

    // println!("You chose {}", game_menu_options[chosen_item])
}

fn create_character(char_name: String) -> Player {
    Player::new_player(char_name)
}

fn generate_boss() -> Boss {
    Boss::create_boss(String::from("Jack"), 500, 10, 2, 100)
}
