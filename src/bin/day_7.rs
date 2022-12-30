use std::{fs, iter::Peekable, rc::Weak};
use my_tree::Node;
use std::rc::Rc;


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
        let mut root_directory: Option<Rc<Node<Dir>>> = None;

        let mut current_directory: Weak<Node<Dir>> = Weak::new();

        while let Some(output) = terminal_output.next() {
            let command = Command::new(output);
            command.execute(terminal_output, &mut current_directory);

            if current_directory.upgrade().is_some() && root_directory.is_none() {
                root_directory = Some(current_directory.upgrade().unwrap());
            }
        }

        match root_directory {
            Some (root_directory) => {
                calculate_directory_total_sizes(&root_directory);
            },
            None => (),
        }
    }
}

fn calculate_directory_total_sizes(root_directory: &Rc<Node<Dir>>) {
    todo!()
}

struct Dir {
    name: String,
    size: usize,
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            size: 0
        }
    }

    fn add_size(&mut self, size: usize) {
        self.size += size;
    }
}

enum Command {
    CD(Dir),
    LS,
}

impl Command {
    fn new(current_line: &str) -> Command {
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

    fn execute(&self, terminal_output: &mut Peekable<std::str::Lines>, current_directory: &mut Weak<Node<Dir>>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_equal() {
        let a = Dir::new("a".to_string());

        let mut b = Dir::new("a".to_string());

        b.add_size(200);

        assert!(a == b);
    }

    #[test]
    fn size() {
        let mut a = Dir::new("a".to_string());

        a.add_size(39);

        assert_eq!(a.size, 39);
    }
}
