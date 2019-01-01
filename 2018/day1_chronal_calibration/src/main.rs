use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut frequency = 0;
    let mut frequencies = HashSet:: new();
    frequencies.insert(frequency);
    let mut status = 0;
    loop {
        let file = File::open("input.txt")?;
        for line in BufReader::new(file).lines() {
            let line_int: i32 = line?.parse().unwrap();
            frequency += line_int;
            if frequencies.contains(& frequency) {
                println!("Answer: {}", frequency);
                status = 1;
                break;
            } else {
                frequencies.insert(frequency);
            }
        }
        if status == 1 {
            break;
        }
    }
    Ok(())
}
