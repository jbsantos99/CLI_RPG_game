use indicatif::ProgressBar;

use crate::utils::delay_in_ms::delay_in_ms;

pub fn update_progbar_msg(progbar: &ProgressBar, hit_value: u32, is_crit: bool) {
    let progbar_original_name = progbar.message();

    let crit_message = if is_crit {
        String::from(" CRITICAL")
    } else {
        String::from("")
    };

    progbar.set_message(
        [
            progbar_original_name.clone(),
            String::from(" -"),
            hit_value.to_string(),
            crit_message,
        ]
        .join(""),
    );

    delay_in_ms(1000);

    progbar.set_message(progbar_original_name);
}

pub fn close_progbars(prog_bars: &[&ProgressBar]) {
    for val in prog_bars {
        val.finish()
    }
}
