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

fn check_surrounding(grid: &Vec<Vec<char>>, row: usize, col: usize, word: &str) -> i32 {
    let mut count = 0;
    if grid[row][col] != word.chars().nth(0).unwrap() {
        return count;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // All 8 directions: right, left, up, down, up-right, up-left, down-right, down-left
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (-1, 0),  // up
        (1, 0),   // down
        (-1, 1),  // up-right
        (-1, -1), // up-left
        (1, 1),   // down-right
        (1, -1),  // down-left
    ];

    // Check each direction
    for (dx, dy) in directions {
        let mut cur_row = row as i32;
        let mut cur_col = col as i32;
        for (i, char) in word.chars().enumerate().skip(1) {
            // Convert current position to signed integers for safe arithmetic
            cur_row = cur_row + dx;
            cur_col = cur_col + dy;

            // Check if the new position is within bounds
            if cur_row >= 0 && cur_row < rows as i32 &&
                cur_col >= 0 && cur_col < cols as i32 {
                // Convert back to usize for array access
                if grid[cur_row as usize][cur_col as usize] != char {
                    break;
                } else if i == word.len() - 1 {
                    count += 1;
                }
            }
        }
    }

    count
}

// Usage in main or another function:
fn process_grid(grid: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            count += check_surrounding(grid, row, col, word)
        }
    }
    count
}

fn main() -> io::Result<()> {
    let grid = read_grid("4.txt")?;
    let word = "XMAS";
    let result = process_grid(&grid, word);
    println!("Found {}: {} times", word, result);

    Ok(())
}