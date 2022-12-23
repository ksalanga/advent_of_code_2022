use std::{fs, iter::Peekable};

fn main() {
    let terminal_output = fs::read_to_string("./inputs/day_7/example1.txt").unwrap();

    // commands:
    // cd: updates and traverses in memory tree structure

    // ls: will read list of items after ls command in terminal output until end of file or new command is reached

    let mut terminal_output: Peekable<std::str::Lines> = terminal_output.lines().peekable();

    while let Some(command) = terminal_output.next() {
        if command.contains("$ ls") {
            read_list(&mut terminal_output);
        }
    }
}

fn read_list(terminal_output: &mut Peekable<std::str::Lines>) {
    // read every line and increment the iterator until we reach a new command $ or end of file (None)

    while let Some(output) = terminal_output.peek() {
        if output.contains("$") {
            return;
        }

        // do something with list output
        println!("{}", terminal_output.next().unwrap());
    }
}
