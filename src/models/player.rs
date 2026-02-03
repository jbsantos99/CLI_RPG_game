use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

#[derive(Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub coins_balance: u32,
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Player", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("hp", &self.hp)?;
        state.serialize_field("attack", &self.attack)?;
        state.serialize_field("coins_balance", &self.coins_balance)?;
        state.serialize_field("defense", &self.defense)?;
        state.end()
    }
}

impl Player {
    pub fn new_player(input_name: String) -> Player {
        Player {
            name: input_name,
            hp: 100,
            attack: 10,
            defense: 1,
            coins_balance: 0,
        }
    }
}
