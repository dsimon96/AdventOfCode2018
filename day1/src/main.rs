use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename)?;

    let mut frequency: i32 = 0;
    
    for line in BufReader::new(file).lines() {
        let change = line.unwrap().parse::<i32>().unwrap();
        frequency += change;
    }

    println!("{}", frequency);
    Ok(())
}
