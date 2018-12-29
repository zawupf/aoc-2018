extern crate aoc_2018;
use aoc_2018::d08::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/08/input.txt").expect("Missing input data");
    let input = content.trim_end();

    println!("Metadata sum: {}", metadata_sum(&input));
    println!("Root value: {}", root_value(&input));
}
