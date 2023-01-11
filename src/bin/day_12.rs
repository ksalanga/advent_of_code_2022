use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<char>>,
    starting_point: Position,
    highest_point: Position,
}

impl HeightMap {
    fn out_of_bounds(&self, position: &Position) -> bool {
        let heightmap_rows = self.map.len() as i32;
        let heightmap_cols = self.map[0].len() as i32;

        let r = position.row;
        let c = position.col;

        r < 0 || r >= heightmap_rows || c < 0 || c >= heightmap_cols
    }

    fn height_difference(&self, current: &Position, other: &Position) -> i32 {
        if self.out_of_bounds(current) || self.out_of_bounds(other) {
            return 10000;
        }

        let current_height = self.map[current.row as usize][current.col as usize] as i32;
        let other_height = self.map[other.row as usize][other.col as usize] as i32;

        other_height - current_height
    }
}

#[derive(Debug)]
struct HeightMapError;

impl FromStr for HeightMap {
    type Err = HeightMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn find_letter(letter: char, heightmap: &Vec<Vec<char>>) -> Position {
            for row in 0..heightmap.len() {
                for col in 0..heightmap[0].len() {
                    if heightmap[row][col] == letter {
                        return Position {
                            row: row as i32,
                            col: col as i32,
                        };
                    }
                }
            }

            Position { row: 0, col: 0 }
        }

        let mut map = Vec::new();
        let lines: Vec<&str> = s.lines().collect();

        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            map.push(chars);
        }

        let starting_point = find_letter('S', &map);
        let highest_point = find_letter('E', &map);

        map[starting_point.row as usize][starting_point.col as usize] = 'a';
        map[highest_point.row as usize][highest_point.col as usize] = 'z';

        Ok(HeightMap {
            map,
            starting_point,
            highest_point,
        })
    }
}

fn shortest_path_to_highest_point(heightmap: &HeightMap) -> Option<usize> {
    let mut steps_from_source: HashMap<Position, usize> = HashMap::new();

    let mut visited: HashSet<Position> = HashSet::new();

    let mut q: VecDeque<Position> = VecDeque::new();

    let starting_point = heightmap.starting_point;
    let highest_point = heightmap.highest_point;

    q.push_front(starting_point);
    steps_from_source.insert(starting_point, 0);
    visited.insert(starting_point);

    while let Some(current) = q.pop_front() {
        if current == highest_point {
            let steps_from_source = *steps_from_source.get(&current).unwrap();
            return Some(steps_from_source);
        }

        let current_steps_from_source = *steps_from_source.get(&current).unwrap();

        for neighbor in get_neighbors(&current, heightmap, &visited) {
            visited.insert(neighbor);
            q.push_back(neighbor);
            steps_from_source.insert(neighbor, current_steps_from_source + 1);
        }
    }
    None
}

fn get_neighbors(
    current: &Position,
    heightmap: &HeightMap,
    visited: &HashSet<Position>,
) -> Vec<Position> {
    let Position { row, col } = current;

    let up = Position {
        row: *row + 1,
        col: *col,
    };
    let down = Position {
        row: *row - 1,
        col: *col,
    };
    let left = Position {
        row: *row,
        col: *col - 1,
    };
    let right = Position {
        row: *row,
        col: *col + 1,
    };

    let neighbors = vec![up, down, left, right];

    neighbors
        .into_iter()
        .filter(|neighbor| {
            !heightmap.out_of_bounds(&neighbor)
                && !visited.contains(&neighbor)
                && heightmap.height_difference(&current, &neighbor) <= 1
        })
        .collect()
}

fn main() {
    let file_path_from_src = "./inputs/day_12/input.txt";
    let mountain: String = fs::read_to_string(file_path_from_src).unwrap();

    let heightmap: HeightMap = mountain.parse().unwrap();

    let shortest_path_to_highest_point = shortest_path_to_highest_point(&heightmap);

    println!(
        "shortest path to mountain top: {}",
        shortest_path_to_highest_point.unwrap()
    );
}

#[cfg(test)]
mod tests {

    use super::*;
}
