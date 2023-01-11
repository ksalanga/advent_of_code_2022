use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::io::Write;
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
    path: HashSet<Position>,
    current: Position,
    heightmap: &mut HeightMap,
    seen_positions: &mut HashMap<Position, Option<usize>>,
) -> Option<usize> {
    if current == heightmap.highest_point {
        return Some(0);
    }

    let r = current.row;
    let c = current.col;

    let up = Position { row: r + 1, col: c };
    let down = Position { row: r - 1, col: c };
    let left = Position { row: r, col: c - 1 };
    let right = Position { row: r, col: c + 1 };

    let neighbors = vec![up, down, left, right];

    let neighbors: Vec<Position> = neighbors
        .into_iter()
        .filter(|neighbor| {
            !heightmap.out_of_bounds(&neighbor)
                && !path.contains(&neighbor)
                && heightmap.height_difference(&current, &neighbor) <= 1
        })
        .collect();

    let mut neighbor_paths: Vec<Option<usize>> = vec![];

    for neighbor in neighbors {
        if seen_positions.contains_key(&neighbor) {
            let shortest_path_to_highest_point = *seen_positions.get(&neighbor).unwrap();
            neighbor_paths.push(shortest_path_to_highest_point);
            continue;
        }

        let mut path = path.clone();
        path.insert(current);
        let shortest_path_to_highest_point =
            shortest_path_to_highest_point(path, neighbor, heightmap, seen_positions);

        neighbor_paths.push(shortest_path_to_highest_point);
    }

    let neighbor_paths: Vec<usize> = neighbor_paths
        .into_iter()
        .filter(|neighbor_path| neighbor_path.is_some())
        .map(|neighbor_path| neighbor_path.unwrap())
        .collect();

    if neighbor_paths.is_empty() {
        seen_positions.insert(current, None);
        return None;
    }

    let shortest_neighbor_path = neighbor_paths.iter().min().unwrap();

    seen_positions.insert(current, Some(shortest_neighbor_path + 1));
    return Some(shortest_neighbor_path + 1);
}

fn main() {
    let file_path_from_src = "./inputs/day_12/input.txt";
    let mountain: String = fs::read_to_string(file_path_from_src).unwrap();

    let mut heightmap: HeightMap = mountain.parse().unwrap();

    let path = HashSet::new();

    let mut seen_positions = HashMap::new();

    let shortest_path_to_highest_point = shortest_path_to_highest_point(
        path,
        heightmap.starting_point,
        &mut heightmap,
        &mut seen_positions,
    );

    for seen_position in seen_positions.keys() {
        heightmap.map[seen_position.row as usize][seen_position.col as usize] = 'X';
    }

    // write_map_to_file(heightmap.map);

    println!(
        "shortest path to mountain top: {}",
        shortest_path_to_highest_point.unwrap()
    );
}

fn write_map_to_file(map: Vec<Vec<char>>) {
    let mut file = fs::File::create("./outputs/day_12/output.txt").unwrap();

    for row in map {
        for ch in row {
            file.write(&[ch as u8]).unwrap();
        }
        file.write(&[b'\n']).unwrap();
    }
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
            path,
            starting_point,
            &mut heightmap,
            &mut seen_positions,
        );
        assert_eq!(shortest_path, Some(2));
    }

    #[test]
    fn find_shortest_path_going_down_and_up() {
        //xyxyz
        //NNNNN
        let mountain = "xyxyE\nNNNNN";
        let mut heightmap: HeightMap = mountain.parse().unwrap();

        let path = HashSet::new();

        let mut seen_positions = HashMap::new();

        let starting_point = Position { row: 0, col: 0 };

        heightmap.map[0][0] = 'x';

        let shortest_path = shortest_path_to_highest_point(
            path,
            starting_point,
            &mut heightmap,
            &mut seen_positions,
        );
        assert_eq!(shortest_path, Some(4));
    }

    #[test]
    fn find_shortest_path_going_up() {
        //waabcdefghijklm
        //xyEyxwvutsrqpon
        let mountain = "waabcdefghijklm\nxyEyxwvutsrqpon";
        let mut heightmap: HeightMap = mountain.parse().unwrap();

        let path = HashSet::new();

        let mut seen_positions = HashMap::new();

        let starting_point = Position { row: 0, col: 0 };

        heightmap.map[0][0] = 'w';

        let shortest_path = shortest_path_to_highest_point(
            path,
            starting_point,
            &mut heightmap,
            &mut seen_positions,
        );
        assert_eq!(shortest_path, Some(3));
    }

    #[test]
    fn find_shortest_path_sliding_down() {
        //yaabcdefghijklm
        //xyEyxwvutsrqpon
        let mountain = "yaabcdefghijklm\nxyEyxwvutsrqpon";
        let mut heightmap: HeightMap = mountain.parse().unwrap();

        let path = HashSet::new();

        let mut seen_positions = HashMap::new();

        let starting_point = Position { row: 0, col: 0 };

        heightmap.map[0][0] = 'y';

        let shortest_path = shortest_path_to_highest_point(
            path,
            starting_point,
            &mut heightmap,
            &mut seen_positions,
        );
        assert_eq!(shortest_path, Some(3));
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
