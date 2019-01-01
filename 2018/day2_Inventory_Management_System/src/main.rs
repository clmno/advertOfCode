use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io::SeekFrom;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    part_one(&file)?;
    file.seek(SeekFrom::Start(0))?;
    part_two(&file)?;
    Ok(())
}

fn part_one(file:& File) -> Result<()> {
    let mut count_twice = 0;
    let mut count_thrice = 0;
    for line in BufReader::new(file).lines() {
        let new_line = line?;
        let new_line = new_line.as_str();
        let line_chars_vec: Vec<char> = new_line.chars().collect();
        let line_chars_vec_cloned = line_chars_vec.clone();
        let mut map: HashMap<char, usize> = HashMap::new();
        for key in line_chars_vec {
            map.entry(key)
                .or_insert(count_char(&line_chars_vec_cloned, key));
        }
        if map.values().any(|&n| n == 2) {
            count_twice += 1;
        }
        if map.values().any(|&n| n == 3) {
            count_thrice += 1;
        }
    }
    println!("Answer 1: {}", count_thrice * count_twice);
    Ok(())
}

fn count_char(s: &Vec<char>, c: char) -> usize {
    s.iter().filter(|&n| *n == c).count()
}

fn part_two(file:& File) -> Result<()> {
    //we need to compare each letter of each word with each letter
    //of every other word
    let ids: Vec<String> = BufReader::new(file).lines()
                            .map(|l| l.expect("Could not parse line"))
                            .collect();
    
    for i in 0..ids.len() {
        for j in i+1..ids.len() {
            if &ids[i].len() != &ids[j].len() {
                continue;
            }
            if let Some(common) = find_common_letters(&ids[i], &ids[j]) {
                println!("Answer 2: {}", common);
                return Ok(())
            }
        }
    }
    Ok(())
}

fn find_common_letters(ids1: &str, ids2: &str) -> Option<String> {
    let mut found_one_wrong = false;
    //use zip to 'Zips up' two iterators into a single iterator of pairs. 
    for (uno,dose) in ids1.chars().zip(ids2.chars()){
        if uno != dose {
            //check if we've previously flaged that a wrong char was found
            if found_one_wrong == true {
                return None;//Return a failed case
            }
            //found one char which is not the same, flag it
            found_one_wrong = true;
        }
    }

    Some(
        ids1.chars().zip(ids2.chars())
        .filter(|&(c1, c2)| c1 == c2)
        .map(|(c,_)| c)
        .collect()
    )
}
