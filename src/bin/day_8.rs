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

struct Coordinates {
    row: usize,
    col: usize
}

struct Height(usize);

fn find_visible_trees(visible_trees: &mut Vec<Vec<bool>>, trees: &Vec<Vec<usize>>) {
    // scan the trees:
    let row_len = trees.len();
    let col_len = trees[0].len();

    // scanning the columns
    // top to bottom
    // bottom to top
    for col in 0..col_len {
        let mut top_side: Vec<(Height, Coordinates)> = Vec::new();
        
        for row in 0..row_len {
            let tree_height = Height(trees[row][col]);
            let tree_coords = Coordinates{row, col};
            
            let tree = (tree_height, tree_coords);
            
            top_side.push(tree);
        }

        find_visible_trees_from_edge(visible_trees, &top_side);

        let bottom_side: Vec<(Height, Coordinates)> = top_side.into_iter().rev().collect();

        find_visible_trees_from_edge(visible_trees, &bottom_side);
    }

    // scanning the rows
    // left to right
    // right to left
    for row in 0..row_len {
        let mut left_side: Vec<(Height, Coordinates)> = Vec::new();
        
        for col in 0..col_len {
            let tree_height = Height(trees[row][col]);
            let tree_coords = Coordinates{row, col};
            
            let tree = (tree_height, tree_coords);
            
            left_side.push(tree);
        }

        find_visible_trees_from_edge(visible_trees, &mut left_side);

        let mut right_side: Vec<(Height, Coordinates)> = left_side.into_iter().rev().collect();

        find_visible_trees_from_edge(visible_trees, &mut right_side);
    }
}

fn find_visible_trees_from_edge(visible_trees: &mut Vec<Vec<bool>>, line_of_trees: &Vec<(Height, Coordinates)>) {
    // skip the front edge but get its height.
    let mut current_tallest_tree_height: usize = line_of_trees[0].0.0;

    // ignore the last edge
    for i in 1..line_of_trees.len() - 1 {
        let current_tree_height: usize = line_of_trees[i].0.0;

        let current_tree_coords: &Coordinates = &line_of_trees[i].1;

        if current_tree_height > current_tallest_tree_height {
            visible_trees[current_tree_coords.row][current_tree_coords.col] = true;
            current_tallest_tree_height = current_tree_height;
        }
    }
}

fn visible_tree_count(visible_trees: &Vec<Vec<bool>>) -> usize {
    // gets the sum of all booleans that are true in the 2d array
    visible_trees.iter().flatten().fold(0, |acc, &visible| acc + (visible as usize))
}