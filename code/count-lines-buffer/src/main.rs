use std::{io::{BufRead, BufReader}, fs::File};

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("../warandpeace.txt").unwrap();
    let buffered_reader = BufReader::new(file);
    println!("Line count: {}", buffered_reader.lines().count());
    println!("Completed in {} ms", now.elapsed().as_millis());
}
