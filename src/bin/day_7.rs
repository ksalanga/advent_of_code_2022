use std::{fs, iter::Peekable};

fn main() {
    let terminal_output = fs::read_to_string("./inputs/day_7/example1.txt").unwrap();

    // commands:
    // cd: updates and traverses in memory tree structure

    // ls: will read list of items after ls command in terminal output until end of file or new command is reached

    let mut terminal_output: Peekable<std::str::Lines> = terminal_output.lines().peekable();

    CommandExecutor::start(&mut terminal_output);
}

struct CommandExecutor {}

impl CommandExecutor {
    fn start(terminal_output: &mut Peekable<std::str::Lines>) {
        while let Some(output) = terminal_output.next() {
            let command = Command::new(output);
            command.execute(terminal_output);
        }
    }
}

struct Dir<'d> {
    parent: Option<&'d Dir<'d>>,
    name: String,
    size: usize,
}

enum Command<'d> {
    CD(Dir<'d>),
    LS,
}

impl<'d> Command<'d> {
    fn new(current_line: &str) -> Command<'d> {
        let mut tokens = current_line.split_whitespace();

        tokens.next();

        if let Some(command) = tokens.next() {
            match command {
                "cd" => {
                    let name = tokens.next();

                    if let None = name {
                        panic!("no directory for cd provided")
                    }

                    return Command::CD(Dir {
                        parent: None,
                        name: name.unwrap().to_string(),
                        size: 0,
                    });
                }
                "ls" => return Command::LS,
                _ => panic!("Command not found"),
            }
        }

        panic!("No command in terminal output");
    }

    fn execute(&self, terminal_output: &mut Peekable<std::str::Lines>) {
        match self {
            Command::CD(dir) => {
                // TODO: create directory nodes (Dir struct) and traverse directory tree with a current directory
                println!("Directory: {}", dir.name)
            }
            Command::LS => {
                // TODO: for any file, add up the size of the file to the current directory
                println!("LS command executed:");
                Self::read(terminal_output);
            }
        }
    }

    fn read(terminal_output: &mut Peekable<std::str::Lines>) {
        // TODO: return a list of strings that represent the non command / non EOF terminal output lines

        // read every line and increment the iterator until we reach a new command $ or end of file (None)

        while let Some(output) = terminal_output.peek() {
            if output.contains("$") {
                return;
            }

            // do something with list output
            println!("{}", terminal_output.next().unwrap());
        }
    }
}
