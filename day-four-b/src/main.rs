use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_grid(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Collect lines into a Vec<Vec<char>>
    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(grid)
}

fn check_surrounding_for_x_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if grid[row][col] != 'A' {
        return false;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let up_row = row as i32 - 1;
    let down_row = row as i32 + 1;
    let back_col = col as i32 - 1;
    let forward_col = col as i32 + 1;

    // Check if any positions are out of bounds
    if up_row < 0 || down_row >= rows as i32 || back_col < 0 || forward_col >= cols as i32 {
        return false;
    }

    // grab the characters
    let up_back = grid[up_row as usize][back_col as usize];
    let down_back = grid[down_row as usize][back_col as usize];
    let up_forward = grid[up_row as usize][forward_col as usize];
    let down_forward = grid[down_row as usize][forward_col as usize];
    let mut leg_1_mas = false;
    let mut leg_2_mas = false;
    if up_back == 'M' && down_forward == 'S' ||
        up_back == 'S' && down_forward == 'M' {
        leg_1_mas = true;
    }
    if down_back == 'M' && up_forward == 'S' ||
        down_back == 'S' && up_forward == 'M' {
        leg_2_mas = true;
    }
    leg_1_mas && leg_2_mas
}

fn process_grid_for_x_mas(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if check_surrounding_for_x_mas(grid, row, col) {
                count += 1;
            }
        }
    }
    count
}

fn main() -> io::Result<()> {
    let grid = read_grid("4.txt")?;
    let result = process_grid_for_x_mas(&grid);
    println!("Found X-MAS: {} times", result);

    Ok(())
}