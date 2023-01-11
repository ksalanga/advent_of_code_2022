use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::rc::Rc;
use std::rc::Weak;
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
    // visited positions hashset
    // <position, steps> hashmap

    // initiate BFS queue

    // starting from the starting point in the heightmap:

    // put the starting point into the queue.
    // put the starting point into the hashmap with steps = 0.

    // for each position in the queue:
    // if position == highest point:
    //      return Some(steps from hashmap)
    // put position in visited set.
    // filter the up, down, left, right neighbor positions that fill the criteria:
    //      - neighbor position isn't out of bounds
    //      - neighbor hasn't been visited
    //      - neighbor position's height difference with current position is <= 1
    // for each filtered neighbor:
    //      - add to BFS queue
    //      - add to position steps hashmap with steps value = current position + 1

    todo!()
}

fn main() {
    let file_path_from_src = "./inputs/day_12/input.txt";
    let mountain: String = fs::read_to_string(file_path_from_src).unwrap();

    let heightmap: HeightMap = mountain.parse().unwrap();

    todo!();
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
