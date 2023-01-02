use std::fs;
use std::vec::Vec;

fn main() {
    let file_path_from_src = "./inputs/day_8/input.txt";
    let contents: String = fs::read_to_string(file_path_from_src).unwrap();

    let lines: Vec<&str> = contents.lines().collect();

    let trees: Vec<Vec<usize>> = parse_grid(lines);

    let n: usize = trees.len();

    let m: usize = trees.get(0).unwrap().len();

    let mut visible_trees: Vec<Vec<bool>> = vec![vec![false; m]; n];
    set_borders_true(&mut visible_trees);

    find_visible_trees(&mut visible_trees, &trees);

    println!("visible trees: {}", visible_tree_count(&visible_trees));
}

fn parse_grid(file_lines: Vec<&str>) -> Vec<Vec<usize>> {
    // Initialize a 2D vector with the same dimensions as the file
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for _ in 0..file_lines.len() {
        grid.push(Vec::new());
    }

    // Parse the numbers and add them to the 2D vector
    for (i, line) in file_lines.iter().enumerate() {
        for c in line.chars() {
            let num: usize = c.to_string().parse().expect("Error parsing number");
            grid[i].push(num);
        }
    }

    grid
}

fn set_borders_true(grid: &mut Vec<Vec<bool>>) {
    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        for j in 0..m {
            if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
                grid[i][j] = true;
            }
        }
    }
}

#[derive(Clone, Default)]
struct Coordinates {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
struct Height(usize);

impl Ord for Height {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Height {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_visible_trees(visible_trees: &mut Vec<Vec<bool>>, trees: &Vec<Vec<usize>>) {
    // scan a line of trees from a specific side/edge (top/bottom side or left/right side):
    let mut scan_sides = |side: &mut Vec<(Height, Coordinates)>| {
        find_visible_trees_from_edge(visible_trees, side);
        side.reverse();
        find_visible_trees_from_edge(visible_trees, side);
    };

    // map the trees into a tree coords 2d array of (Height, Coordinates)
    let mut tree_coords_left_right: Vec<Vec<(Height, Coordinates)>> =
        map_to_height_and_coords(get_coords(trees));

    // scan the visible tree rows (left to right and right to left) by going through the tree coords rows.
    tree_coords_left_right.iter_mut().for_each(&mut scan_sides);

    // swap the rows and the columns of the mapped tree cords matrix.
    // now, going through the rows of this swapped matrix will be going through the "columns" of the original matrix since the coordinates remain the same during the swap.
    let mut tree_coords_top_bottom = swap_rows_and_columns(&tree_coords_left_right);

    // view the visible tree columns (top to bottom and bottom to top) by going through these new mapped rows.
    tree_coords_top_bottom.iter_mut().for_each(&mut scan_sides);
}

// useful matrix function:
// returns same 2d array but the values are now a tuple (Value, row coordinate, column coordinate)
fn get_coords<T>(arr: &Vec<Vec<T>>) -> Vec<Vec<(T, usize, usize)>>
where
    T: Copy,
{
    arr.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &x)| (x, i, j))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn map_to_height_and_coords(
    coords_arr: Vec<Vec<(usize, usize, usize)>>,
) -> Vec<Vec<(Height, Coordinates)>> {
    coords_arr
        .iter()
        .map(|row| {
            row.iter()
                .map(|tree| {
                    (
                        Height(tree.0),
                        Coordinates {
                            row: tree.1,
                            col: tree.2,
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn swap_rows_and_columns<T>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone + Default,
{
    let mut result = vec![vec![T::default(); arr.len()]; arr[0].len()];
    for (i, row) in arr.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            result[j][i] = val.clone();
        }
    }
    result
}

fn find_visible_trees_from_edge(
    visible_trees: &mut Vec<Vec<bool>>,
    line_of_trees: &Vec<(Height, Coordinates)>,
) {
    // skip the front edge but get its height.
    let mut current_tallest_tree_height: Height = line_of_trees[0].0;

    // ignore the last edge
    for i in 1..line_of_trees.len() - 1 {
        let current_tree_height: Height = line_of_trees[i].0;

        let current_tree_coords: &Coordinates = &line_of_trees[i].1;

        if current_tree_height > current_tallest_tree_height {
            visible_trees[current_tree_coords.row][current_tree_coords.col] = true;
            current_tallest_tree_height = current_tree_height;
        }
    }
}

fn visible_tree_count(visible_trees: &Vec<Vec<bool>>) -> usize {
    // gets the sum of all booleans that are true in the 2d array
    visible_trees
        .iter()
        .flatten()
        .fold(0, |acc, &visible| acc + (visible as usize))
}
