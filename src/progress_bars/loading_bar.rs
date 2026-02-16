use std::time::Duration;

use indicatif::ProgressBar;

use crate::utils::delay_in_ms::delay_in_ms;

pub fn loading_bar(time_in_ms: u32, msg: String) {
    let loading_bar = ProgressBar::new_spinner();
    loading_bar.enable_steady_tick(Duration::from_millis(100));

    loading_bar.set_message(msg);

    delay_in_ms(time_in_ms as u64)
}
