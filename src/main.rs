use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    let file = File::open("data.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{contents}");
    let elves = contents.split("\n\n");
    // println!("length of array {}", )
    Ok(())
}
