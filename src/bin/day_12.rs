use std::fs;
use std::hash::Hash;
use std::str::FromStr;

// New Plan: BFS version of shortest path
// Previous implementation used DFS which is a bit more confusing and requires memoization and recursion
// BFS version of shortest path is simpler to understand and create.
// - Create graph of Position nodes that are reachable from source
// - Do a BFS of graph from source to target
// - starting and current node is source with steps = 0 from source.
// - from a curr Node, its non visited neighboring node gets the curr node's steps + 1.
// - first visit of target is min steps from source to target.

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

impl FromStr for HeightMap {
    type Err = HeightMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

struct Node {}

fn shortest_path_to_highest_point(graph: &Node) -> Option<usize> {
    todo!()
}

fn main() {
    let file_path_from_src = "./inputs/day_12/input.txt";
    let mountain: String = fs::read_to_string(file_path_from_src).unwrap();

    let mut heightmap: HeightMap = mountain.parse().unwrap();

    todo!();
    let shortest_path_to_highest_point = shortest_path_to_highest_point(&Node {});

    println!(
        "shortest path to mountain top: {}",
        shortest_path_to_highest_point.unwrap()
    );
}

#[cfg(test)]
mod tests {

    use super::*;
}
