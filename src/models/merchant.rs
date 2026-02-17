use core::fmt;
use std::{cell::Cell, fs};

use serde::{Deserialize, Serialize};

use crate::{
    menus::merchant::launch_merchant_menu,
    models::player::Player,
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
pub struct Merchant {
    name: String,
    price: u32,
    description: String,
    modifier: Modifier,
    is_bought: Cell<bool>,
}

impl fmt::Display for Merchant {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{},  \n {} \n cost: ${}",
            self.name, self.description, self.price
        )
    }
}

impl Merchant {
    pub fn create_merchant_list() {
        let health_1 = Merchant {
            name: String::from("Health Elixir 1"),
            price: 200,
            description: String::from("Will permenently increase your health by 100 points"),
            modifier: Modifier {
                mod_type: ModifierType::TotalHealth,
                value: (100),
            },
            is_bought: Cell::new(false),
        };

        let health_2 = Merchant {
            name: String::from("Health Elixir 2"),
            price: 300,
            description: String::from("Will permenently increase your health by 200 points"),
            modifier: Modifier {
                mod_type: ModifierType::TotalHealth,
                value: 200,
            },
            is_bought: Cell::new(false),
        };

        let crit_hit_1 = Merchant {
            name: String::from("Deadeye"),
            price: 150,
            description: String::from("This will permenently increase critical hit chance by 10%"),
            modifier: Modifier {
                mod_type: ModifierType::CritHitChance,
                value: 10,
            },
            is_bought: Cell::new(false),
        };

        let crit_hit_2 = Merchant {
            name: String::from("Eagle Eye"),
            price: 90,
            description: String::from("This will permenently increase critical hit chance by 5%"),
            modifier: Modifier {
                mod_type: ModifierType::CritHitChance,
                value: 5,
            },
            is_bought: Cell::new(false),
        };

        let attack_1 = Merchant {
            name: String::from("Attack 1"),
            price: 130,
            description: String::from(
                "Increases min. and max. attack by 5. (eg. 5 - 10  =>  10 - 15) ",
            ),
            modifier: Modifier {
                mod_type: ModifierType::Attack,
                value: 5,
            },
            is_bought: Cell::new(false),
        };

        let attack_2 = Merchant {
            name: String::from("Attack 2"),
            price: 180,
            description: String::from(
                "Increases min. and max. attack by 5. (eg. 8 - 10  =>  13 - 18) ",
            ),
            modifier: Modifier {
                mod_type: ModifierType::Attack,
                value: 8,
            },
            is_bought: Cell::new(false),
        };

        let parsed_items = serde_json::to_string_pretty(&[
            &health_1,
            &health_2,
            &attack_1,
            &attack_2,
            &crit_hit_1,
            &crit_hit_2,
        ])
        .expect("Failed to parse merchant items");

        if !fs::exists("game_data/merchant_list")
            .expect("Error checking game data: failed to check if file exists.")
        {
            fs::write("game_data/merchant_list", parsed_items)
                .expect("failed to save merchant list");
            println!("Saved merchant items");
        }
    }

    pub fn get_items() -> Vec<Merchant> {
        let check_file = fs::exists("game_data/merchant_list").expect("Error on 'get_items'");

        if !check_file {
            Merchant::create_merchant_list();
        }

        let raw_json = fs::read_to_string("game_data/merchant_list")
            .expect("Failed to fetch merchant list json");

        let items_json: Vec<Merchant> =
            serde_json::from_str(raw_json.as_str()).expect("Failed to parse merchant items");

        items_json
    }

    pub fn get_item_by_id(id: usize) -> Merchant {
        let items = Merchant::get_items();

        return items[id].clone();
    }

    pub fn purchase(self, player: &Player) {
        if self.price > player.coins_balance.get() {
            clear_terminal();

            println!("Not enough cash, Stranger!");
            delay_in_ms(2000);

            launch_merchant_menu()
        }

        match self.modifier.mod_type {
            ModifierType::TotalHealth => {
                let old_hp = player.hp.get();
                player.hp.set(old_hp + self.modifier.value);
            }
            ModifierType::CritHitChance => {
                let old_val = player.crit_chance.get();
                player.crit_chance.set(old_val + self.modifier.value);
            }
            ModifierType::Attack => {
                let (min, max) = player.attack_range.get();
                let att_increment = self.modifier.value;

                player
                    .attack_range
                    .set((min + att_increment, max + att_increment));
            }
        }

        player.decr_coins(self.price);
        player.save();

        self.is_bought.set(true);

        delay_in_ms(1000);
        delay_in_ms(1000);

        launch_merchant_menu();
    }
}
