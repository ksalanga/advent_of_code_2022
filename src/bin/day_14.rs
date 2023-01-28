use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::result::Result;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Element {
    Air,
    Rock,
    Sand,
}

struct Cave {
    starting_sand_map_coords: Coordinates,
    map: Vec<Vec<Element>>,
}

struct InfiniteFall {}

impl Cave {
    fn new(rock_paths_line_endpoints: Vec<Vec<Coordinates>>) -> Cave {
        let mut rock_coords: Vec<Coordinates> = rock_paths_line_endpoints
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<Coordinates>>();

        // getting x length: largest x value - smallest x value
        rock_coords.sort_by(|a, b| a.x.cmp(&b.x));
        let smallest_x = rock_coords.get(0).unwrap().x;
        let largest_x = rock_coords.get(rock_coords.len() - 1).unwrap().x;
        let x_len = largest_x - smallest_x + 1;

        rock_coords.sort_by(|a, b| a.y.cmp(&b.y));
        let largest_y = rock_coords.get(rock_coords.len() - 1).unwrap().y;
        let y_len = largest_y + 1;

        let mut map = vec![vec![Element::Air; x_len as usize]; y_len as usize];

        let starting_sand_coords = Coordinates {
            x: 0,
            y: 500 - smallest_x,
        };

        // TODO: Place rocks into cave given rock paths

        // transform the rock paths' line endpoint coordinates into 2d vector coordinates
        let rock_paths_points_map_coordinates: Vec<Vec<Coordinates>> = rock_paths_line_endpoints
            .iter()
            .map(|rock_path_points| {
                rock_path_points
                    .iter()
                    .map(|coordinate| Coordinates {
                        x: coordinate.x - smallest_x,
                        y: coordinate.y,
                    })
                    .collect()
            })
            .collect();

        // transform rock paths which consists of line endpoint map coordinates into the entire points of the line
        // we're basically "drawing the line" given our rock path line endpoints
        let rock_map_coordinates: Vec<Coordinates> = rock_paths_points_map_coordinates
            .into_iter()
            .map(|rock_path_points| Self::draw_rock_path_lines(rock_path_points))
            .flatten()
            .collect();

        for Coordinates { x, y } in rock_map_coordinates {
            // map column = x coordinate - smallest x coordinate
            // map row = y coordinate
            map[y as usize][x as usize] = Element::Rock;
        }

        Cave {
            starting_sand_map_coords: starting_sand_coords,
            map,
        }
    }

    fn out_of_bounds(&self, coords: &Coordinates) -> bool {
        let Coordinates { x: row, y: col } = *coords;

        row < 0 || row >= self.map.len() as i32 || col < 0 || col >= self.map[0].len() as i32
    }

    fn fall(&self, sand_coords: &Coordinates) -> Result<Coordinates, InfiniteFall> {
        let Coordinates { x, y } = sand_coords;

        let element_below = Coordinates { x: *x + 1, y: *y };

        if self.out_of_bounds(&element_below) {
            return Err(InfiniteFall {});
        }

        match self.map[element_below.x as usize][element_below.y as usize] {
            Element::Air => return self.fall(&element_below),
            _rock => {
                let mut bottom_left_element = Coordinates {
                    x: element_below.x,
                    y: element_below.y - 1,
                };

                if self.out_of_bounds(&bottom_left_element) {
                    return Err(InfiniteFall {});
                }

                if self.map[bottom_left_element.x as usize][bottom_left_element.y as usize]
                    == Element::Air
                {
                    return self.fall(&mut bottom_left_element);
                }

                let mut bottom_right_element = Coordinates {
                    x: element_below.x,
                    y: element_below.y + 1,
                };

                if self.out_of_bounds(&bottom_right_element) {
                    return Err(InfiniteFall {});
                }

                if self.map[bottom_right_element.x as usize][bottom_right_element.y as usize]
                    == Element::Air
                {
                    return self.fall(&mut bottom_right_element);
                }

                return Ok(sand_coords.clone());
            }
        }
    }

    fn drop_sand(&mut self) -> Result<(), InfiniteFall> {
        match self.fall(&self.starting_sand_map_coords) {
            Ok(coords) => {
                self.map[coords.x as usize][coords.y as usize] = Element::Sand;
                return Ok(());
            }
            _ => return Err(InfiniteFall {}),
        }
    }

    fn draw_rock_path_lines(rock_path_line_endpoints: Vec<Coordinates>) -> Vec<Coordinates> {
        // given each line endpoint, create the line of coordinates
        let mut line_coordinates = vec![];

        let mut rock_path_line_endpoints = rock_path_line_endpoints.iter().peekable();

        while let Some(endpoint_1) = rock_path_line_endpoints.next() {
            if let Some(endpoint_2) = rock_path_line_endpoints.peek() {
                line_coordinates.append(&mut Self::get_line_coordinates(&endpoint_1, &endpoint_2));
            } else {
                break;
            }
        }

        line_coordinates
    }

    fn get_line_coordinates(
        endpoint_1: &Coordinates,
        endpoint_2: &Coordinates,
    ) -> Vec<Coordinates> {
        let mut line_endpoints: Vec<Coordinates> = vec![endpoint_1.clone(), endpoint_2.clone()];

        if endpoint_1.x == endpoint_2.x {
            // vertical line
            line_endpoints.sort_by(|a, b| a.y.cmp(&b.y));

            let x = line_endpoints[0].x;
            let lowest_y = line_endpoints[0].y;
            let highest_y = line_endpoints[1].y;

            for y in lowest_y + 1..highest_y {
                line_endpoints.push(Coordinates { x, y });
            }
        } else {
            // horizontal line
            line_endpoints.sort_by(|a, b| a.x.cmp(&b.x));

            let y = line_endpoints[0].y;
            let lowest_x = line_endpoints[0].x;
            let highest_x = line_endpoints[1].x;

            for x in lowest_x + 1..highest_x {
                line_endpoints.push(Coordinates { x, y });
            }
        }

        line_endpoints
    }

    fn sand_count(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|element| **element == Element::Sand)
            .count()
    }

    #[allow(dead_code)]
    fn y_len(&self) -> i32 {
        self.map.len() as i32
    }

    #[allow(dead_code)]
    fn x_len(&self) -> i32 {
        self.map[0].len() as i32
    }
}

#[derive(Clone, Default, PartialEq)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CoordinatesParseError;

impl fmt::Display for CoordinatesParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "coordinate parsing error")
    }
}

impl std::convert::From<ParseIntError> for CoordinatesParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for Coordinates {
    type Err = CoordinatesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(",");

        let x = s.next().ok_or(CoordinatesParseError)?;
        let x: i32 = x.parse()?;

        let y = s.next().ok_or(CoordinatesParseError)?;
        let y: i32 = y.parse()?;

        Ok(Coordinates { x, y })
    }
}

fn main() {
    let file_path_from_src = "./inputs/day_14/input.txt";
    let input: String = fs::read_to_string(file_path_from_src).unwrap();
    let mut rock_paths = vec![];

    for rock_path in input.lines() {
        let rock_path: Vec<Coordinates> = rock_path
            .split(" -> ")
            .map(|coord| coord.parse::<Coordinates>().unwrap())
            .collect();

        rock_paths.push(rock_path);
    }

    let mut cave = Cave::new(rock_paths);

    while let Ok(()) = cave.drop_sand() {}

    println!("Sand count: {}", cave.sand_count());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Coordinates as coords;

    #[test]
    fn get_cave_boundaries() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let cave = Cave::new(rock_paths);

        assert_eq!(cave.x_len(), 10);
        assert_eq!(cave.y_len(), 10);
    }

    #[test]
    fn get_uneven_cave_boundaries() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let cave = Cave::new(rock_paths);

        assert_eq!(cave.x_len(), 9);
        assert_eq!(cave.y_len(), 10);
    }

    #[test]
    fn starting_sand_map_coords() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let cave = Cave::new(rock_paths);

        assert!(cave.starting_sand_map_coords == Coordinates { x: 0, y: 6 });
    }

    #[test]
    fn rocks_placed_in_cave() {
        let mut expected_cave_map = vec![vec![Element::Air; 10]; 10];

        for col in 2..5 {
            expected_cave_map[6][col] = Element::Rock;
        }

        for row in 4..7 {
            expected_cave_map[row][4] = Element::Rock;
        }

        expected_cave_map[4][9] = Element::Rock;

        for row in 4..10 {
            expected_cave_map[row][8] = Element::Rock;
        }

        for col in 0..9 {
            expected_cave_map[9][col] = Element::Rock;
        }

        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let cave = Cave::new(rock_paths);

        assert!(cave.map == expected_cave_map);
    }

    #[test]
    fn drop_one_sand() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let mut cave = Cave::new(rock_paths);

        cave.drop_sand();

        assert!(cave.map[8][6] == Element::Sand);
    }

    #[test]
    fn drop_sand_left() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let mut cave = Cave::new(rock_paths);

        cave.drop_sand();
        cave.drop_sand();

        assert!(cave.map[8][6] == Element::Sand);
        assert!(cave.map[8][5] == Element::Sand);
    }

    #[test]
    fn drop_sand_right() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let mut cave = Cave::new(rock_paths);

        cave.drop_sand();
        cave.drop_sand();
        cave.drop_sand();

        assert!(cave.map[8][6] == Element::Sand);
        assert!(cave.map[8][5] == Element::Sand);
        assert!(cave.map[8][7] == Element::Sand);
    }

    #[test]
    fn sand_count() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let mut cave = Cave::new(rock_paths);

        cave.drop_sand();
        cave.drop_sand();
        cave.drop_sand();

        assert!(cave.sand_count() == 3);
    }

    #[test]
    fn aoc_example_sand_count() {
        let rock_path_1 = vec![
            coords::new(498, 4),
            coords::new(498, 6),
            coords::new(496, 6),
        ];
        let rock_path_2 = vec![
            coords::new(503, 4),
            coords::new(502, 4),
            coords::new(502, 9),
            coords::new(494, 9),
        ];

        let rock_paths = vec![rock_path_1, rock_path_2];
        let mut cave = Cave::new(rock_paths);

        while let Ok(()) = cave.drop_sand() {}

        assert!(cave.sand_count() == 24);
    }

    #[test]
    fn parse_coordinate() {
        let coord_str = "498,3";

        let coord = coord_str.parse::<Coordinates>();

        assert!(coord == Ok(Coordinates { x: 498, y: 3 }));
    }
}
