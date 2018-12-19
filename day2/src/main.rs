use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::env;
use std::collections::HashMap;

fn part1(file: File) -> Result<()> {
    let mut two_counts = 0;
    let mut three_counts = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let mut counts: HashMap<char, i32> = HashMap::new();

        for letter in line.chars() {
            let letter_count = counts.entry(letter).or_default();

            *letter_count += 1;
        }

        if counts.values().any(|p| *p == 2) { two_counts += 1; }
        if counts.values().any(|p| *p == 3) { three_counts += 1; }
    }

    println!("{}", two_counts * three_counts);
    Ok(())
}

fn part2(file: File) -> Result<()> {
    let mut prev: Vec<String> = Vec::new();

    let mut result: Option<(String, String)> = None;

    for line in BufReader::new(file).lines() {
        let this = line.unwrap();

        let other = prev.iter()
            .find_map(|other| {
                if (*other).chars()
                    .zip(this.chars())
                    .filter(|(x, y)| x != y)
                    .count() == 1 {
                    Some(other.clone())
                } else {
                    None
                }
            });

        if other.is_some() {
            result = Some((this, other.unwrap()));
            break;
        }

        prev.push(this);
    };

    let (a, b) = result.unwrap();

    let in_common: String = a.chars().zip(b.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect();

    println!("{}", in_common);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let part = &args[1];
            let filename = &args[2];

            let file = File::open(filename)?;

            match &part[..] {
                "1" => part1(file),
                "2" => part2(file),
                _ => panic!("Unrecognized part")
            }
        }
        _ => panic!("Must provide a part number and path to input")
    }
}
