#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct PreviousStep {
    position: Position,
    height: char,
    count: u32,
}

// from any current position in the mountain,
// we take a single step (increment our step count)

// we recurse through our neighbors that aren't ones that we came from.
//      (if we allow going through where we came from we could infinitely recurse going back and forth between those two spots)
// we provide our neighbor our current height and coords into their previous height and coords part of the recursive function.

// after we got the amount of steps from all of our valid returned neighbors,
// take the neighbor with the least amount of steps.
// return to sender.

// we stop recursing from a current position based on the condition:
// - we are out of bounds from the coordinates
// - we are a height difference greater than 1 from our previous neighbor
// return None

// we stop recursing from a current position based on the condition:
// height == E
// return Some(the previous step count).

// we don't recurse through a neighbor from a curent position based on these conditions:
// - previous neighbor's coordinates

// fn step(prev: Option<PreviousStep>, current_position: Position, mountain: &Vec<Vec<char>>) -> Option<u32>
fn step_counts(
    previous_step: Option<PreviousStep>,
    current_position: Position,
    mountain: &Vec<Vec<char>>,
) -> Option<u32> {
    // mountain grid width = columns
    let mountain_width = mountain[0].len() as i32;
    // mountain grid height = rows
    let mountain_height = mountain.len() as i32;

    // x is width (going left to right of mountain grid)
    let x = current_position.x;

    // y is height (going up and down of mountain grid)
    let y = current_position.y;

    if x < 0 || x >= mountain_width || y < 0 || y >= mountain_height {
        return None;
    }

    if mountain[y as usize][x as usize] == 'E' {
        return Some(previous_step.unwrap().count);
    }

    if let Some(previous_step) = &previous_step {
        let previous_height =
            mountain[previous_step.position.y as usize][previous_step.position.x as usize] as u32;

        let current_height =
            mountain[current_position.y as usize][current_position.x as usize] as u32;

        if current_height.abs_diff(previous_height) > 1 {
            return None;
        }
    }

    let previous_step_count = match &previous_step {
        Some(previous_step) => previous_step.count,
        None => 0,
    };

    let current_step_count = previous_step_count + 1;

    let mut valid_steps = Vec::new();

    let current_step = PreviousStep {
        position: current_position,
        height: mountain[current_position.y as usize][current_position.x as usize],
        count: current_step_count,
    };

    let mut next_positions: Vec<Position> = Vec::new();

    for y in vec![-1, 1] {
        let next_position = Position {
            x: current_position.x,
            y: current_position.y + y,
        };

        if let Some(previous_step) = &previous_step {
            if next_position == previous_step.position {
                continue;
            }
        }

        next_positions.push(next_position);
    }

    for x in vec![-1, 1] {
        let next_position = Position {
            x: current_position.x + x,
            y: current_position.y,
        };

        if let Some(previous_step) = &previous_step {
            if next_position == previous_step.position {
                continue;
            }
        }

        next_positions.push(next_position);
    }

    for next_position in next_positions {
        // println!(
        //     "Current Position: {:?}, Next Position: {:?}",
        //     current_position, next_position
        // );
        if let Some(step_count) = step_counts(Some(current_step), next_position, mountain) {
            valid_steps.push(step_count);
        }
    }

    if valid_steps.is_empty() {
        return None;
    }

    valid_steps.sort();
    return Some(valid_steps[0]);
}

fn main() {
    todo!()
}

fn parse_mountain(s: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let lines: Vec<&str> = s.lines().collect();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        result.push(chars);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_shortest_path() {
        let mountain = "ab\naE";

        let mountain = parse_mountain(mountain);

        let current_position = Position { x: 0, y: 0 };
        let shortest_path = step_counts(None, current_position, &mountain);

        assert_eq!(shortest_path, Some(2));
    }

    #[test]
    fn cant_reach_mountain_top() {
        let mountain = "ac\ncE";

        let mountain = parse_mountain(mountain);

        let current_position = Position { x: 0, y: 0 };
        let shortest_path = step_counts(None, current_position, &mountain);

        assert_eq!(shortest_path, None);
    }
}
