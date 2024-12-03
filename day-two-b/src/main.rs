use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = match line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
        {
            Ok(nums) => nums,
            Err(e) => {
                eprintln!("Error parsing number: {}", e);
                std::process::exit(1);
            }
        };
        if is_safe(numbers, false) {
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

fn is_safe_change(first: i32, second: i32) -> bool {
    let diff = second - first;
    diff.abs() <= 3 && diff.abs() >=1
}

fn is_safe(report: Vec<i32>, second_pass: bool) -> bool {
    let mut previous: i32 = 0;
    // walk the line and check everything
    let mut increasing: bool = false;
    let mut decreasing: bool = false;
    for (i, current) in report.iter().enumerate() {
        if i == 1 {
            // initial increasing / decreasing determination
            increasing = is_increasing(previous, *current);
            decreasing = is_decreasing(previous, *current);
            // if we don't change it's unsafe and we're done
            if !is_safe_change(previous, *current) {
                return if second_pass {
                    false
                } else {
                    is_safe_removing_at_index(report, i)
                }
            }
        }
        if i > 1 {
            // Now do all the tests
            if increasing && is_decreasing(previous, *current) {
                return if second_pass {
                    false
                } else {
                    is_safe_removing_at_index(report, i)
                }
            }
            if decreasing && is_increasing(previous, *current) {
                return if second_pass {
                    false
                } else {
                    is_safe_removing_at_index(report, i)
                }
            }
            if !is_safe_change(previous, *current) {
                return if second_pass {
                    false
                } else {
                    is_safe_removing_at_index(report, i)
                }
            }
        }
        previous = *current;
    }
    // we got to the
    if second_pass {
        println!("Safe on second pass: {:?}", report);
    }
    true
}


fn remove_at_index_preserve(numbers: Vec<i32>, index: usize) -> Vec<i32> {
    let mut result = numbers.clone();
    result.remove(index);
    result
}

fn is_safe_removing_at_index(numbers: Vec<i32>, index: usize) -> bool {
    println!("## original: {:?}, removing {}", numbers, index);
    let numbers_copy = numbers.clone();
    let changed_this = remove_at_index_preserve(numbers, index);
    let this = is_safe(changed_this, true);
    let changed_previous = remove_at_index_preserve(numbers_copy, index - 1);
    let previous = is_safe(changed_previous, true);
    this || previous
}