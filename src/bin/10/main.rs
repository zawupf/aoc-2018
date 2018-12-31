extern crate aoc_2018;
use aoc_2018::d10::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/10/input.txt").expect("Missing input data");
    let input = content.trim_end().lines().collect::<Vec<_>>();

    let (message, seconds) = message_and_seconds(&input);
    println!("Message after {} seconds:\n{}", seconds, message);
}
