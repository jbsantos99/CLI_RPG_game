pub fn line_spacer(break_amounts: usize) {
    let line_spaces = vec!["\n"; break_amounts];
    let formated = line_spaces.join("");

    println!("{}", formated)
}
