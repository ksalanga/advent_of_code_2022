#[derive(Clone, Copy)]
enum Element {
    Air,
    Rock,
    Sand,
}

struct Cave {
    vertical_map: Vec<Vec<Element>>,
}

impl Cave {
    fn new(rock_paths: Vec<Vec<Coordinates>>) -> Cave {
        let mut rock_coords: Vec<Coordinates> = rock_paths
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

        let vertical_map = vec![vec![Element::Air; x_len as usize]; y_len as usize];

        // TODO: Place rocks into cave given rock paths

        // transform the rock coordinates into 2d vector coordinates
        let rock_paths_points_map_coordinates: Vec<Vec<Coordinates>> = rock_paths_points
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

        Cave { map }
    }

    fn draw_rock_path_lines(rock_path_line_endpoints: Vec<Coordinates>) -> Vec<Coordinates> {
        todo!()
        // given each endpoint, create the line of coordinates
    }

    fn y_len(&self) -> i32 {
        self.vertical_map.len() as i32
    }

    fn x_len(&self) -> i32 {
        self.vertical_map[0].len() as i32
    }
}

#[derive(Clone, Default)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }
}

fn main() {
    todo!();
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
    fn rocks_placed_in_cave() {
        todo!()
    }
}
