use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("3.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut do_mul = true;
    let mut total: i32 = 0;
    let commands_re = Regex::new(r"((do)\(\)|(don't)\(\)|(mul)\(\d{1,3},\d{1,3}\))").unwrap();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for line in reader.lines() {
        // read line by line and do a regex to get things to multiply
        for (_, [the_match, command]) in commands_re.captures_iter(line?.as_str()).map(|c| c.extract()) {
            match command {
                "do" => do_mul = true,
                "don't" => do_mul = false,
                "mul" => {
                    if do_mul {
                        let the_ops = mul_re.captures(the_match).unwrap();
                        let product = the_ops[1].parse::<i32>().unwrap() * the_ops[2].parse::<i32>().unwrap();
                        total += product;
                    }
                },
                _ => println!("no match")
            }
        }
    }
    println!("{}", total);
    Ok(())
}