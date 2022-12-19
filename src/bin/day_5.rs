use std::fs;

fn main() {
    let cargo_crane = fs::read_to_string("./inputs/day_5/example1.txt").unwrap();

    let cargo_crane_components: Vec<&str> = cargo_crane.split("\n\r\n").collect();
    // Cargo Crane has access to Stacks of Crates

    let stacks: Stacks<char> = Stacks::new(cargo_crane_components[0]);

    assert_eq!(stacks.count, 3);

    // Cargo Crane has access to rearrangement procedure
}

struct Stacks<T> {
    count: i32,
    stacks: Vec<Stack<T>>,
}

impl<T> Stacks<T> {
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

        Stacks {
            count,
            stacks: Self::place_crates_on_stacks(stacks, count),
        }
    }

    fn place_crates_on_stacks<'a>(
        stacks: impl Iterator<Item = &'a str>,
        count: i32,
    ) -> Vec<Stack<T>> {
        // Getting Supplies in the Crates of the Stacks

        // Parsing Each row of string crates to place into stacks

        stacks.for_each(|crates| {
            let mut supplies = crates.chars().skip(1);

            for i in 0..count {
                let supply = supplies.next().unwrap();
                supplies.skip(3);
                println!("Supply: {}", supply);
            }
        });

        Vec::new()
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
