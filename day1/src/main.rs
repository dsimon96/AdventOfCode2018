use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::env;
use std::collections::HashSet;

fn part1(file: File) -> Result<()> {
    let mut frequency: i32 = 0;
    
    for line in BufReader::new(file).lines() {
        let change = line.unwrap().parse::<i32>().unwrap();
        frequency += change;
    }

    println!("{}", frequency);
    Ok(())
}

fn part2(file: File) -> Result<()> {
    let mut changes: Vec<i32> = Vec::new();

    for line in BufReader::new(file).lines() {
        changes.push(line.unwrap().parse::<i32>().unwrap());
    }

    let mut freq: i32 = 0;
    let mut seen: HashSet<i32> = HashSet::new();

    let mut done = false;
    while !done {
        for change in &changes {
            seen.insert(freq);
            freq += change;

            if seen.contains(&freq) {
                done = true;
                break
            }
        }
    }

    println!("{}", freq);
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
                    _ => Ok(())
                }
        },
        _ => Ok(())
    }


}
