use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_arg = env::args().nth(1);
    let empty = String::from("");
    let arg = raw_arg.unwrap_or(empty);

    if arg.is_empty() {
        println!("No path provided.");
        exit(0);
    }

    let f = File::open(&arg);

    if f.is_err() {
        println!("Cannot find the specified file.");
        exit(0);
    }

    let reader = BufReader::new(f?);

    for line in reader.lines() {
        println!("{}", &line?);
    }

    Ok(())
}
