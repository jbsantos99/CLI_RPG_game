In this case, I can mutate the msg even while passing an read reference of progress bar, because 
the mutation happens internally. This is interior mutability in RUST.

Its a design choice, it all depends if the creator used &self or &mut self when creating it.


pub fn update_progbar_msg(progbar: &ProgressBar, msg: String) {
    progbar.set_message(msg);
}


For that I can use Cell, but its only safe to use if Im running single thread applications.

Cell implemente basic set and get values for internal mutations like this.

