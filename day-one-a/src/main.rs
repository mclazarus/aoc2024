use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let Ok(num1) = numbers[0].parse::<i32>() {
                first_column.push(num1);
            }
            if let Ok(num2) = numbers[1].parse::<i32>() {
                second_column.push(num2);
            }
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    first_column.sort();
    second_column.sort();
    let mut total_distance = 0;
    for (num1, num2) in first_column.iter().zip(second_column.iter()) {
        // calculate the distance between num1 and num2 (absolute value)
        let difference = num1 - num2;
        let distance = difference.abs();
        total_distance += distance;
    }
    println!("Total Distance: {:7}", total_distance);

    Ok(())
}