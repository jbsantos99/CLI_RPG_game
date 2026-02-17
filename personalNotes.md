### notes


#### Decisions

attack and defence are ranges as (u32, u32), but the struct that holds 
the MerchantItem value is a single u32. To set it I made it so we use the single
value will be added to both number in the (u32, u32) cell. This is single
solution that could change in the future.


In this case, I can mutate the msg even while passing an read reference of progress bar, because 

the mutation happens internally. This is interior mutability in RUST.

Its a design choice, it all depends if the creator used &self or &mut self when creating it.


pub fn update_progbar_msg(progbar: &ProgressBar, msg: String) {
    progbar.set_message(msg);
}


For that I can use Cell, but its only safe to use if Im running single thread applications.

Cell implemente basic set and get values for internal mutations like this.



-- number of items in the json files --
while building the menus I have to fetch bosses and list of merchant items. I dont like creating dynamic size arrays, but fixed ones require const values know a compile time. 
is it worth to check json file and modify my rust file with a script to the correct lenght?
 ++ the function can be const ( check const functions )


## having multiple save files
question: if I have multiple game files for player, how can I know which on is it?
answer: when I chose a file on game launch I can create a copy of that one and put it to lets say
a "buffer" folder, all the methods should read from that. I can also save the this buffer should save the
info to, like "target_file: a".




## what is the difference between .expect and .expect_err?




