use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::exit;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if is_safe(numbers) {
            safe_count += 1;
        }
    }

    println!("Safe reports: {:7}", safe_count);

    Ok(())
}

fn is_increasing(first: i32, second: i32) -> bool {
    second > first
}

fn is_decreasing(first: i32, second: i32) -> bool {
    first > second
}

fn is_unchanged(first: i32, second: i32) -> bool {
    first == second
}

fn is_safe_change(first: i32, second: i32) -> bool {
    let diff = second - first;
    diff.abs() <= 3
}

fn is_safe(report: Vec<&str>) -> bool {
    let mut previous: i32 = 0;
    // walk the line and check everything
    let mut increasing: bool = false;
    let mut decreasing: bool = false;
    for (i, current_str) in report.iter().enumerate() {
        if let Ok(current) = current_str.parse::<i32>() {
            if i == 1 {
                // initial increasing / decreasing determination
                increasing = is_increasing(previous, current);
                decreasing = is_decreasing(previous, current);
                // if we don't change it's unsafe and we're done
                if is_unchanged(previous, current) {
                    return false;
                }
                if !is_safe_change(previous, current) {
                    return false;
                }
            }
            if i > 1 {
                // Now do all the tests
                if increasing && is_decreasing(previous, current) {
                    return false;
                }
                if decreasing && is_increasing(previous, current) {
                    return false;
                }
                if is_unchanged(previous, current) {
                    return false;
                }
                if !is_safe_change(previous, current) {
                    return false;
                }
            }
            previous = current;
        } else {
            eprintln!("{} exiting fails to parse number", current_str);
            exit(1);
        }
    }
    // we got to the
    true
}