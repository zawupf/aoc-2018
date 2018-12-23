extern crate aoc_2018;
use aoc_2018::d05::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/05/input.txt").expect("Missing input data");
    let input = content.trim_end();

    println!("Remaining units count: {}", remaining_units(input).len());
    println!("Shortest polymer length: {}", shortest_polymer_length(input));
}
