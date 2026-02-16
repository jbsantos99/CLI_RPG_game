use std::{thread, time};

pub fn delay_in_ms(ms_amnt: u64) {
    let sleep_time = time::Duration::from_millis(ms_amnt);
    thread::sleep(sleep_time);
}
