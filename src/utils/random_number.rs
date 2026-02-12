use rand::Rng;

pub fn random_number(tuple_range: (u32, u32)) -> u32 {
    // had a proble with this, the bug was actually on the boss generation function
    if tuple_range.0 >= tuple_range.1 {
        panic!("Error on  random_number: invalid tuple.")
    }

    let mut rng = rand::rng();
    rng.random_range(tuple_range.0..tuple_range.1)
}
