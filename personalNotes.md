In this case, I can mutate the msg even while passing an read reference of progress bar, because 
the mutation happens internally. This is interior mutability in RUST.

pub fn update_progbar_msg(progbar: &ProgressBar, msg: String) {
    progbar.set_message(msg);
}
