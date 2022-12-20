use std::fs;

fn main() {
    let cargo_crane = fs::read_to_string("./inputs/day_5/example1.txt").unwrap();

    let cargo_crane_components: Vec<&str> = cargo_crane.split("\n\r\n").collect();

    // Cargo Crane has access to Stacks of Crates
    let mut stacks: Stacks = Stacks::new(cargo_crane_components[0]);

    assert_eq!(stacks.count, 3);
    assert_eq!(stacks.stacks[0].pop(), Some('N'));
    assert_eq!(stacks.stacks[0].pop(), Some('Z'));
    assert_eq!(stacks.stacks[1].pop(), Some('D'));
    assert_eq!(stacks.stacks[1].pop(), Some('C'));
    assert_eq!(stacks.stacks[1].pop(), Some('M'));
    assert_eq!(stacks.stacks[2].pop(), Some('P'));

    // Cargo Crane has access to rearrangement procedure
    move_crates(cargo_crane_components[1], &mut stacks);
}

fn move_crates(procedure: &str, stacks: &mut Stacks) {
    procedure.lines().for_each(|instructions| {
        let move_instructions = MoveInstructions::new(instructions);
    })
}

struct MoveInstructions {
    amount_of_crates: i32,
    from_stack: i32,
    to_stack: i32,
}

impl MoveInstructions {
    fn new(instructions: &str) -> MoveInstructions {
        // Split the instructions string by white space and alphanumerics
        let mut instructions = instructions
            .split(|c| char::is_alphabetic(c) || char::is_whitespace(c))
            .filter(|s| !s.is_empty());

        // first number instance in instructions (could be greater than a single digit) is amount

        // second number instance in instructions (could be greater than a single digit) is from stack.

        // third number instance in instructions (could be greater than a single digit) is to stack.
        MoveInstructions {
            amount_of_crates: instructions.next().unwrap().parse().unwrap(),
            from_stack: instructions.next().unwrap().parse().unwrap(),
            to_stack: instructions.next().unwrap().parse().unwrap(),
        }
    }
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

        let mut stacks: Vec<Stack<char>> = (0..count).map(|_| Stack::new()).collect();

        // Parsing Each row of string crates to place into stacks
        crates.for_each(|crates| {
            crates
                .chars()
                .skip(1) // Skip first index of characters in OG String which is either a beginning of crate: [ or empty. Next iterated value will be supplies if crate or empty if empty.
                .enumerate() // Enumerate the Characters starting index @ 0 from the first possible supplies.
                .filter(|(i, supplies)| supplies.is_alphabetic() && i % 4 == 0) // Filter only possible supplies.
                // Starting from an index where supplies possibly are, the next supplies character will be 4 indices ahead.
                // ex: [S1] [S2] S1 to S2 is index 0 (starting from first possible supply) to index 4.
                .map(|(i, supplies)| (i / 4, supplies)) // Map OG String Indices to Designated Stacks Indices
                // Since supplies go in stacks left to right, the Character enumeration index for a supply is % 4, and the String supplies index is 4x the designated stack index, the mapped designated stacks index is just i / 4.
                .for_each(|(i, supplies)| stacks[i].push(supplies));
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
