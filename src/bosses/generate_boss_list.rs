use crate::{
    models::bosses::{Boss, BOSS_NAMES},
    state_managers::save_boss_array::save_generated_boss_array,
    utils::random_number::random_number,
};

pub fn generate_bosses() {
    let mut new_boss_array: Vec<Boss> = Vec::new();

    for (index, &item) in BOSS_NAMES.iter().enumerate() {
        println!("Generating {:?}", &item);

        let base_mult = index as u32 + 1;

        let raw_boss = Boss::create_boss(
            String::from(item),
            random_number((100, 120)) * base_mult,
            (
                random_number((5, 10)) * base_mult,
                random_number((10, 15)) * base_mult,
            ),
            (
                random_number((1, 3)) * base_mult,
                random_number((3, 6)) * base_mult,
            ),
            random_number((1 * base_mult, 5 * base_mult)),
            random_number((40, 80)) * base_mult,
            false,
        );

        new_boss_array.push(raw_boss);
    }

    println!("Boss Generating Finished!");
    println!("This is the boss array {:#?}", new_boss_array);

    println!("Saving state...");
    save_generated_boss_array(&new_boss_array);
}
