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
    fn shift(&mut self, direction: Direction) {
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

    fn move_head(&mut self, direction: Direction) {
        todo!()
    }

    fn is_diagonal(&self) -> bool {
        self.head.position.x != self.tail.position.x && self.head.position.y != self.tail.position.y
    }

    fn is_long(&self) -> bool {
        (self.head.position.x - self.tail.position.x).abs() >= 2
            || (self.head.position.y - self.tail.position.y).abs() >= 2
    }
}

// methods:
// move(direction)
// store that the rope was attached diagonally or not
// head moves(direction)
// after head moves, is one of the x and y distances between head and tail >= 2?
// if yes:
// tail moves(direction)
// if was diagonally attached:
// if head moved up/down (y direction):
// make tail's x position = head's x position
// else if head move left/right (x direction):
// make tail's y position = head's y position

// calculate if head and tail are attached diagonally()

// calculate if x and y distances are >= 2()

// start:
// create a tail position set.

// parse the lines.
// while we have lines:
// split line by whitespace
// create a new direction for line[0]
// times to move in that direction = lines[1]
// move

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
