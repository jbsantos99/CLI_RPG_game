use std::{thread, time};

pub fn delay_in_ms() {
    let sleep_time = time::Duration::from_millis(1000);
    thread::sleep(sleep_time);
}
