use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("3.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut total: i32 = 0;
    for line in reader.lines() {
        // read line by line and do a regex to get things to multiply
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for (_, [op1, op2]) in re.captures_iter(line?.as_str()).map(|c| c.extract()) {
            let product = op1.parse::<i32>().unwrap() * op2.parse::<i32>().unwrap();
            total += product;
        }
    }
    println!("{}", total);
    Ok(())
}