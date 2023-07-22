#![warn(clippy::pedantic)]
fn main() {
    let numbers = (0..100).collect::<Vec<i32>>();
    for i in 0 .. numbers.len() {
        println!("{}", numbers[i]);
    }

    // The right way
    //for i in &numbers {
    //    println!("{i}");
    //}
}
