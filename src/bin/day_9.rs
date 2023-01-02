use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
// enum direction:
// up, down, left, right
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(input: &str) -> Direction {
        match input {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid input"),
        }
    }
}

// struct rope:
// fields:
// head and tail

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Knot {
    position: Position,
}

impl Knot {
    fn new() -> Knot {
        Knot {
            position: Position { x: 0, y: 0 },
        }
    }
    fn shift(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }
}

struct Rope {
    head: Knot,
    tail: Knot,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Knot::new(),
            tail: Knot::new(),
        }
    }

    #[allow(dead_code)]
    fn new_starting_position(position: Position) -> Rope {
        Rope {
            head: Knot { position },
            tail: Knot { position },
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let was_diagonal = self.is_diagonal();

        self.head.shift(direction);

        if self.is_long() {
            self.tail.shift(direction);

            if was_diagonal {
                match direction {
                    Direction::Up | Direction::Down => self.tail.position.x = self.head.position.x,
                    Direction::Left | Direction::Right => {
                        self.tail.position.y = self.head.position.y
                    }
                }
            }
        }
    }

    fn is_diagonal(&self) -> bool {
        self.head.position.x != self.tail.position.x && self.head.position.y != self.tail.position.y
    }

    fn is_long(&self) -> bool {
        (self.head.position.x - self.tail.position.x).abs() >= 2
            || (self.head.position.y - self.tail.position.y).abs() >= 2
    }
}

// start:
// create a tail position set.

// parse the lines.
// while we have lines:
// split line by whitespace
// create a new direction for line[0]
// times to move in that direction = lines[1]
// move

macro_rules! loopn {
    ($n:expr, $body:block) => {
        for _ in 0..$n {
            $body
        }
    };
}

fn main() {
    let file_path_from_src = "./inputs/day_9/input.txt";
    let move_instructions: String = fs::read_to_string(file_path_from_src).unwrap();

    let mut rope = Rope::new();

    let mut tail_touched_coords: HashSet<Position> = HashSet::new();
    tail_touched_coords.insert(rope.tail.position);

    move_instructions.lines().for_each(|line| {
        let input: Vec<&str> = line.split_whitespace().collect();

        let move_direction: Direction = Direction::new(input[0]);

        let times_to_move: i32 = input[1].parse().unwrap();

        loopn!(times_to_move, {
            rope.move_head(&move_direction);
            tail_touched_coords.insert(rope.tail.position);
        })
    });

    println!("{}", tail_touched_coords.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touched_coords() {
        let mut touched_coords: HashSet<Position> = HashSet::new();

        touched_coords.insert(Position { x: 0, y: 0 });
        touched_coords.insert(Position { x: 0, y: 0 });

        assert_eq!(touched_coords.len(), 1);
    }

    #[test]
    fn touched_coords_2() {
        let mut touched_coords: HashSet<Position> = HashSet::new();

        touched_coords.insert(Position { x: 0, y: 0 });
        touched_coords.insert(Position { x: 0, y: 1 });

        assert_eq!(touched_coords.len(), 2);
    }

    #[test]
    fn new_rope_is_not_diagonal() {
        let rope = Rope::new();

        assert!(!rope.is_diagonal());
    }

    #[test]
    fn horizontal_rope_is_not_diagonal() {
        let mut rope = Rope::new();

        rope.head.position.x = 1;

        assert!(!rope.is_diagonal());
    }

    #[test]
    fn vertical_rope_is_not_diagonal() {
        let mut rope = Rope::new();

        rope.head.position.y = 1;

        assert!(!rope.is_diagonal());
    }

    #[test]
    fn diagonal_rope_is_not_diagonal() {
        let mut rope = Rope::new();

        rope.head.position.x = 1;
        rope.head.position.y = 1;

        assert!(rope.is_diagonal());
    }

    #[test]
    fn new_rope_is_not_long() {
        let rope = Rope::new();

        assert!(!rope.is_long());
    }

    #[test]
    fn diagonal_length_1_rope_is_not_long() {
        let mut rope = Rope::new();

        rope.head.position.x = 1;
        rope.head.position.y = 1;

        assert!(!rope.is_long());
    }

    #[test]
    fn diagonal_length_2_rope_is_long() {
        let mut rope = Rope::new();

        rope.head.position.x = 2;
        rope.head.position.y = 1;

        assert!(rope.is_long());
    }

    #[test]
    fn move_new_rope_up_1_tail_stays() {
        let mut rope = Rope::new();

        rope.move_head(&Direction::Up);

        assert!(rope.tail.position.x == 0);
        assert!(rope.tail.position.y == 0);
    }

    #[test]
    fn move_new_rope_up_2_tail_moves_up_1() {
        let mut rope = Rope::new();

        rope.move_head(&Direction::Up);
        rope.move_head(&Direction::Up);

        assert!(rope.tail.position.x == 0);
        assert!(rope.tail.position.y == 1);
    }

    #[test]
    fn move_new_rope_down_2_tail_moves_down_1() {
        let mut rope = Rope::new_starting_position(Position { x: 2, y: 2 });

        rope.move_head(&Direction::Down);
        rope.move_head(&Direction::Down);

        assert!(rope.tail.position.x == 2);
        assert!(rope.tail.position.y == 1);
    }

    #[test]
    fn move_new_rope_right_2_tail_moves_right_1() {
        let mut rope = Rope::new();

        rope.move_head(&Direction::Right);
        rope.move_head(&Direction::Right);

        assert!(rope.tail.position.x == 1);
        assert!(rope.tail.position.y == 0);
    }

    #[test]
    fn move_new_rope_left_2_tail_moves_left_1() {
        let mut rope = Rope::new_starting_position(Position { x: 2, y: 2 });

        rope.move_head(&Direction::Left);
        rope.move_head(&Direction::Left);

        assert!(rope.tail.position.x == 1);
        assert!(rope.tail.position.y == 2);
    }

    #[test]
    fn move_new_rope_diagonally_tail_stays() {
        let mut rope = Rope::new();

        rope.move_head(&Direction::Up);
        rope.move_head(&Direction::Right);

        assert!(rope.tail.position.x == 0);
        assert!(rope.tail.position.y == 0);
    }

    #[test]
    fn move_diagonal_rope_up_tail_follows() {
        let mut diagonal_rope = Rope::new();

        diagonal_rope.move_head(&Direction::Up);
        diagonal_rope.move_head(&Direction::Right);

        diagonal_rope.move_head(&Direction::Up);

        assert!(diagonal_rope.tail.position.x == 1);
        assert!(diagonal_rope.tail.position.y == 1);
    }

    #[test]
    fn move_diagonal_rope_right_tail_follows() {
        let mut diagonal_rope = Rope::new_starting_position(Position { x: 1, y: 1 });

        diagonal_rope.move_head(&Direction::Up);
        diagonal_rope.move_head(&Direction::Right);

        diagonal_rope.move_head(&Direction::Right);

        assert!(diagonal_rope.tail.position.x == 2);
        assert!(diagonal_rope.tail.position.y == 2);
    }
}
