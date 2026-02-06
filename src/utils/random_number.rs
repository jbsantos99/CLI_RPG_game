use rand::Rng;

pub fn random_number(start: u32, end: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(start..end)
}
