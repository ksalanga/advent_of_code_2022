use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
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

    fn height_difference(&self, current: &Position, other: &Position) -> u32 {
        if self.out_of_bounds(current) || self.out_of_bounds(other) {
            return 10000;
        }

        let current_height = self.map[current.row as usize][current.col as usize] as u32;
        let other_height = self.map[other.row as usize][other.col as usize] as u32;

        current_height.abs_diff(other_height)
    }

    // if shortest_path_map has a None value @ a position:
    //      shortest_path hasn't calculated a position in the heightmap yet
    // if shortest_path_map has Some value @ a position:
    //      if it's negative: shortest_path @ position has been calculated but it found no paths to the highest point. dead end.
    //      if it's positive: shortest_path @position is some shortest_path.
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

fn shortest_path_to_highest_point(
    steps: usize,
    path: HashSet<Position>,
    current: Position,
    heightmap: &mut HeightMap,
    seen_positions: &mut HashMap<Position, Option<usize>>,
) -> Option<usize> {
    if current == heightmap.highest_point {
        return Some(steps);
    }

    let r = current.row;
    let c = current.col;

    let up = Position { row: r + 1, col: c };
    let down = Position { row: r - 1, col: c };
    let left = Position { row: r, col: c - 1 };
    let right = Position { row: r, col: c + 1 };

    let neighbors = vec![up, down, left, right];

    let mut neighbors_to_explore = vec![];

    for neighbor in neighbors {
        if !heightmap.out_of_bounds(&neighbor)
            && !path.contains(&neighbor)
            && heightmap.height_difference(&current, &neighbor) <= 1
        {
            neighbors_to_explore.push(neighbor);
        }
    }

    let mut neighbor_paths: Vec<Option<usize>> = vec![];

    for neighbor in neighbors_to_explore {
        let mut path = path.clone();
        path.insert(current);

        if seen_positions.contains_key(&neighbor) {
            neighbor_paths.push(*seen_positions.get(&neighbor).unwrap());
        } else {
            let neighbor_shortest_path_to_highest_point = shortest_path_to_highest_point(
                steps + 1,
                path,
                neighbor,
                heightmap,
                seen_positions,
            );

            neighbor_paths.push(neighbor_shortest_path_to_highest_point);
        }
    }

    let mut neighbor_paths: Vec<usize> = neighbor_paths
        .iter()
        .filter(|neighbor_path_length| neighbor_path_length.is_some())
        .map(|neighbor_path_length| neighbor_path_length.unwrap())
        .collect();

    if neighbor_paths.is_empty() {
        seen_positions.insert(current, None);
        return None;
    }

    // put the shortest of neighbor_paths into the shortest_path_map.
    neighbor_paths.sort();
    seen_positions.insert(current, Some(neighbor_paths[0]));
    return Some(neighbor_paths[0]);
}

fn main() {
    let file_path_from_src = "./inputs/day_12/example.txt";
    let mountain: String = fs::read_to_string(file_path_from_src).unwrap();

    let mut heightmap: HeightMap = mountain.parse().unwrap();

    let path = HashSet::new();

    let mut seen_positions = HashMap::new();

    let shortest_path_to_highest_point = shortest_path_to_highest_point(
        0,
        path,
        heightmap.starting_point,
        &mut heightmap,
        &mut seen_positions,
    );

    println!(
        "shortest path to mountain top: {}",
        shortest_path_to_highest_point.unwrap()
    );
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn find_shortest_path() {
        let mountain = "xy\nyE";
        let mut heightmap: HeightMap = mountain.parse().unwrap();

        let path = HashSet::new();

        let mut seen_positions = HashMap::new();

        let starting_point = Position { row: 0, col: 0 };

        heightmap.map[0][0] = 'x';

        let shortest_path = shortest_path_to_highest_point(
            0,
            path,
            starting_point,
            &mut heightmap,
            &mut seen_positions,
        );
        assert_eq!(shortest_path, Some(2));
    }

    #[test]
    fn map() {
        let mut map: HashMap<Position, i32> = HashMap::new();

        let p0 = Position { row: 0, col: 0 };
        let p1 = Position { row: 0, col: 1 };
        map.insert(p0, 0);

        assert!(!map.contains_key(&p1));
        assert!(map.contains_key(&p0));
    }
}
