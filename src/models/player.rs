#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub defense: u32,
}

impl Player {
    pub fn new_player() -> Player {
        Player {
            name: String::from("test"),
            hp: 100,
            defense: 10,
        }
    }
}
