use std::{cell::Cell, fs};

use dialoguer::Select;
use serde::{Deserialize, Serialize};

use crate::{
    menus::boss_menu::launch_boss_menu,
    models::player_models::Player,
    player::getters::get_player_stats,
    state_managers::character_creation::save_new_character,
    utils::{clear_terminal::clear_terminal, delay_in_ms::delay_in_ms},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ModifierType {
    TotalHealth,
    Attack,
    CritHitChance,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modifier {
    mod_type: ModifierType,
    value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerchantItem {
    name: String,
    price: u32,
    description: String,
    modifier: Modifier,
    is_bought: Cell<bool>,
}

impl MerchantItem {
    pub fn create_merchant_list() {
        let health_1 = MerchantItem {
            name: String::from("Health Elixir 1"),
            price: 100,
            description: String::from("Will permenently increase your health by 100 points"),
            modifier: Modifier {
                mod_type: ModifierType::TotalHealth,
                value: (100),
            },
            is_bought: Cell::new(false),
        };

        let health_2 = MerchantItem {
            name: String::from("Health Elixir 2"),
            price: 300,
            description: String::from("Will permenently increase your health by 150 points"),
            modifier: Modifier {
                mod_type: ModifierType::TotalHealth,
                value: (150),
            },
            is_bought: Cell::new(false),
        };

        // let attack_1 = MerchantItem {
        //     name: String::from("Attack Potion 1"),
        //     price: 300,
        //     description: String::from("This will permenently increase you attack to 20 - 25"),
        //     modifier: Modifier {
        //         mod_type: ModifierType::Attack,
        //         value: (150),
        //     },
        //     is_bought: Cell::new(false),
        // };
        //
        // let crit_hit_1 = MerchantItem {
        //     name: String::from("Deadeye"),
        //     price: 300,
        //     description: String::from("This will permenently increase critical hit chance to 30%"),
        //     modifier: MerchantItemModifier::TotalHealth,
        //     is_bought: Cell::new(false),
        // };
        //
        //

        let parsed_items = serde_json::to_string_pretty(&[&health_1, &health_2])
            .expect("Failed to parse merchant items");

        if !fs::exists("game_data/merchant_list")
            .expect("Error checking game data: failed to check if file exists.")
        {
            fs::create_dir("game_data").expect("Failed to save merchant list");
            fs::write("game_data/merchant_list", parsed_items).expect("ss");
            println!("Saved merchant items");
        }
    }

    pub fn get_items() -> Vec<MerchantItem> {
        let raw_json = fs::read_to_string("game_data/merchant_list")
            .expect("Failed to fetch merchant list json");

        let items_json: Vec<MerchantItem> = serde_json::from_str(raw_json.as_str()).expect("dd");
        items_json
    }

    pub fn get_item_by_id(id: usize) -> MerchantItem {
        let items = MerchantItem::get_items();

        return items[id].clone();
    }

    pub fn get_item_names() -> Vec<String> {
        let mut merchant_item_list_names = vec![];

        for val in MerchantItem::get_items() {
            merchant_item_list_names.push(val.name.clone());
        }

        merchant_item_list_names.push(String::from("--- Back ---"));

        return merchant_item_list_names;
    }

    pub fn purchase(self, player: &Player) {
        if self.price > player.coins_balance.get() {
            println!("Not enough cash, Stranger!");
            delay_in_ms();

            launch_merchant_menu()
        }

        let old_hp = player.hp.get();
        player.hp.set(old_hp + self.modifier.value);

        player.save();
        self.is_bought.set(true);
        println!("Your HP is now {}", player.hp.get());
        delay_in_ms();

        launch_merchant_menu();
    }
}

pub fn launch_merchant_menu() {
    clear_terminal();

    let merchant_menu_options = MerchantItem::get_item_names();

    let player = get_player_stats();

    let chosen_item = Select::new()
        .with_prompt("What are you buying?")
        .default(0)
        .items(&merchant_menu_options)
        .interact()
        .unwrap();

    match chosen_item {
        item_id => {
            if item_id == merchant_menu_options.len() - 1 {
                launch_main_menu();
            }

            MerchantItem::get_item_by_id(item_id).purchase(Player::get_info(&player));
        }
    }
}

pub fn launch_main_menu() {
    let game_menu_options = vec!["Fight", "Merchant", "Quit Game"];

    let chosen_item = Select::new()
        .with_prompt("Menu")
        .default(0)
        .items(&game_menu_options)
        .interact()
        .unwrap();

    // let game_menu_options = vec!["Fight", "Weapon Store", "Quit Game"];
    match game_menu_options[chosen_item] {
        "Fight" => launch_boss_menu(),
        "Merchant" => launch_merchant_menu(),
        "Quit Game" => println!("Exiting Game"),
        &_ => todo!(),
    }
}
