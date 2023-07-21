use std::fs::read_to_string;

fn main() {
    let now = std::time::Instant::now();
    let war_and_peace = read_to_string("../warandpeace.txt").unwrap();
    println!("Line count: {}", war_and_peace.lines().count());
    println!("Completed in {} ms", now.elapsed().as_millis());
}
