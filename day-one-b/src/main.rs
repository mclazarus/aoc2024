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
    let mut total_similarity_score = 0;
    for left in first_column.iter() {
        // search for number of times left occurs in second_column
        // multiply number by that and add to total similarity score
        let mut count = 0;
        for right in second_column.iter() {
            if left == right {
                count += 1;
            }
            if right > left {
                break;
            }
        }
        total_similarity_score += left * count;
    }
    println!("Similarity Score: {:7}", total_similarity_score);

    Ok(())
}