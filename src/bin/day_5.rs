use std::fs;

fn main() {
    let cargo_crane = fs::read_to_string("./inputs/day_5/example1.txt").unwrap();

    let cargo_crane_components: Vec<&str> = cargo_crane.split("\n\r\n").collect();
    // Cargo Crane has access to Stacks of Crates

    let stacks: Stacks = Stacks::new(cargo_crane_components[0]);

    assert_eq!(stacks.count, 3);

    // Cargo Crane has access to rearrangement procedure
}

struct Stacks {
    count: i32,
    stacks: Vec<Stack<char>>,
}

impl Stacks {
    // Parse the original stacks string input:
    // Ex:
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    fn new(stacks: &str) -> Self {
        let mut stacks = stacks.lines().rev();

        let count: i32 = stacks
            .next()
            .unwrap()
            .split_whitespace()
            .count()
            .try_into()
            .unwrap();

        let crates = stacks;

        Stacks {
            count,
            stacks: Self::place_on_stacks(crates, count),
        }
    }

    fn place_on_stacks<'a>(crates: impl Iterator<Item = &'a str>, count: i32) -> Vec<Stack<char>> {
        // Getting Supplies in the Crates of the Stacks

        let mut stacks: Vec<Stack<char>> = Vec::with_capacity(count.try_into().unwrap());
        // Parsing Each row of string crates to place into stacks

        crates.for_each(|crates| {
            crates
                .chars()
                .skip(1)
                .enumerate()
                .filter(|(i, _)| i % 4 == 0)
                .for_each(|(i, supplies)| {
                    println!("Stack: {}, Supply: {}", i / 4, supplies);
                    // TODO: Place Supplies / Crate (char) in a Stack @ index i / 4 of Vector<Stack<Char>>
                    if supplies.is_alphabetic() {
                        stacks[i / 4].push(supplies);
                    }
                });
        });

        stacks
    }
}

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}
