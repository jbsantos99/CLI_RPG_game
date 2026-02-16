use indicatif::ProgressBar;

use crate::progress_bars::update_progbar_msg::update_progbar_msg;

pub fn create_battle_bars() {}

pub fn update_hp_on_hit(prog_bar: &ProgressBar, hit_taken: u32, is_crit: bool) {
    prog_bar.dec(hit_taken as u64);

    update_progbar_msg(prog_bar, hit_taken, is_crit);
}
