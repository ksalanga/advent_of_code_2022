use std::{fs, iter::Peekable};
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

        let mut current_directory: Option<Rc<Node<Dir>>> = None;

        while let Some(output) = terminal_output.next() {
            let command = Command::new(output);

            // TODO: Finish executing all commands
            command.execute(terminal_output, &mut current_directory);

            if root_directory.is_none() && current_directory.is_some() {
                root_directory = Some(Rc::clone(&current_directory.as_ref().unwrap()));
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

    fn execute(&self, terminal_output: &mut Peekable<std::str::Lines>, current_directory: &mut Option<Rc<Node<Dir>>>) {
        match self {
            Command::CD(dir) => {                
                match dir.name.as_str() {
                    ".." => move_up(current_directory),
                    "/" => move_to_root(current_directory),
                    target_dir => move_to(target_dir, current_directory),
                }

                fn move_up(current_directory: &mut Option<Rc<Node<Dir>>>) {
                    if let Some(directory) = current_directory {
                        if let Some(parent) = directory.get_parent().upgrade() {
                            *current_directory = Some(parent);
                        }
                    }
                }

                fn move_to_root(current_directory: &mut Option<Rc<Node<Dir>>>) {
                    // empty tree
                    if let None = current_directory {
                        return *current_directory = Some(Node::new(Dir::new("/".to_string())))
                    }

                    while let Some(directory) = current_directory {
                        // reached root
                        if let None = directory.get_parent().upgrade() {
                            return
                        }

                        move_up(current_directory);
                    }
                }

                fn move_to(target_dir_name: &str, current_directory: &mut Option<Rc<Node<Dir>>>) {
                    if let Some(directory) = current_directory {
                        let target_dir = Dir::new(target_dir_name.to_string());

                        if let Some(directory) = directory.get_child(target_dir) {
                            if let Some(directory) = directory.upgrade() {
                                return *current_directory = Some(Rc::clone(&directory));
                            }
                        }

                        println!("cannot cd into directory: {}. this directory has not been listed and does not exist.", target_dir_name);
                    }
                }
            }
            Command::LS => {
                // TODO: for any file, add up the size of the file to the current directory
                todo!();
                println!("LS command executed:");
                Self::read(terminal_output);
            }
        }
    }

    fn read(terminal_output: &mut Peekable<std::str::Lines>) {
        // TODO: return a list of strings that represent the non command / non EOF terminal output lines

        // read every line and increment the iterator until we reach a new command $ or end of file (None)
        let mut lines = vec![];

        while let Some(output) = terminal_output.peek() {
            if output.contains("$") {
                break;
            }

            // do something with list output
            lines.push(terminal_output.next().unwrap());
        }

        lines
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

    #[test]
    fn cd_dot_dot() {
        use std::ptr;

        let command = "$ cd ..";

        let cd = Command::new(command);

        let mut terminal_output = "line1\nline2\nline3\n".lines().into_iter().peekable();

        // execute() Command

        let a = Node::new(Dir::new("/".to_string()));

        let b = Node::new(Dir::new("b".to_string()));

        let mut current_directory = Some(Rc::clone(&b));

        a.add_child(&a, b);

        cd.execute(&mut terminal_output, &mut current_directory);

        assert!(ptr::eq(a.as_ref(), current_directory.unwrap().as_ref()));
    }

    #[test]
    fn cd_root_from_leaf() {
        use std::ptr;

        let command = "$ cd /";

        let cd = Command::new(command);

        let mut terminal_output = "line1\nline2\nline3\n".lines().into_iter().peekable();

        // execute() Command

        let a = Node::new(Dir::new("/".to_string()));

        let b = Node::new(Dir::new("b".to_string()));
        
        let c = Node::new(Dir::new("c".to_string()));

        let mut current_directory = Some(Rc::clone(&c));

        b.add_child(&b, c);

        a.add_child(&a, b);

        cd.execute(&mut terminal_output, &mut current_directory);

        assert!(ptr::eq(a.as_ref(), current_directory.unwrap().as_ref()));
    }

    #[test]
    fn cd_root_from_root() {
        use std::ptr;

        let command = "$ cd /";

        let cd = Command::new(command);

        let mut terminal_output = "line1\nline2\nline3\n".lines().into_iter().peekable();

        // execute() Command

        let a = Node::new(Dir::new("/".to_string()));

        let mut current_directory = Some(Rc::clone(&a));

        cd.execute(&mut terminal_output, &mut current_directory);

        assert!(ptr::eq(a.as_ref(), current_directory.unwrap().as_ref()));
    }

    #[test]
    fn cd_root_from_empty_tree() {
        let command = "$ cd /";

        let cd = Command::new(command);

        let mut terminal_output = "line1\nline2\nline3\n".lines().into_iter().peekable();

        let mut current_directory = None;

        cd.execute(&mut terminal_output, &mut current_directory);

        assert!(current_directory.is_some());
    }

    #[test]
    fn cd_down() {
        use std::ptr;

        let command = "$ cd c";

        let cd = Command::new(command);

        let mut terminal_output = "line1\nline2\nline3\n".lines().into_iter().peekable();

        let a = Node::new(Dir::new("/".to_string()));

        let b = Node::new(Dir::new("b".to_string()));
        
        let c = Node::new(Dir::new("c".to_string()));

        let c_observer = Rc::clone(&c);

        let mut current_directory = Some(Rc::clone(&b));

        b.add_child(&b, c);

        a.add_child(&a, b);

        cd.execute(&mut terminal_output, &mut current_directory);

        assert!(ptr::eq(c_observer.as_ref(), current_directory.unwrap().as_ref()));
    }

    #[test]
    fn cd_down_from_root() {
        use std::ptr;

        let command = "$ cd b";

        let cd = Command::new(command);

        let mut terminal_output = "line1\nline2\nline3\n".lines().into_iter().peekable();

        let a = Node::new(Dir::new("/".to_string()));

        let b = Node::new(Dir::new("b".to_string()));
        
        let c = Node::new(Dir::new("c".to_string()));

        let b_observer = Rc::clone(&b);

        let mut current_directory = Some(Rc::clone(&a));

        b.add_child(&b, c);

        a.add_child(&a, b);

        cd.execute(&mut terminal_output, &mut current_directory);

        assert!(ptr::eq(b_observer.as_ref(), current_directory.unwrap().as_ref()));
    }

    #[test]
    fn cd_nonexistent_child() {
        use std::ptr;

        let command = "$ cd fake";

        let cd = Command::new(command);

        let mut terminal_output = "line1\nline2\nline3\n".lines().into_iter().peekable();

        let a = Node::new(Dir::new("/".to_string()));

        let b = Node::new(Dir::new("b".to_string()));
        
        let c = Node::new(Dir::new("c".to_string()));

        let mut current_directory = Some(Rc::clone(&a));

        b.add_child(&b, c);

        a.add_child(&a, b);

        cd.execute(&mut terminal_output, &mut current_directory);

        assert!(ptr::eq(a.as_ref(), current_directory.unwrap().as_ref()));
    }
    
}
