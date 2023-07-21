use std::{io::{BufRead, BufReader}, fs::File};
use memmap::MmapOptions;

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("../warandpeace.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let buffered_reader = BufReader::new(&mmap[..]);
    println!("Line count: {}", buffered_reader.lines().count());
    println!("Completed in {} ms", now.elapsed().as_millis());
}
