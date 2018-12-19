extern crate aoc_2018;
use aoc_2018::d01::*;
use aoc_2018::read_file;

fn main() {
    let input = read_file("src/bin/01/input.txt")
        .expect("Missing input data")
        .split_whitespace()
        .map(|value| value.parse::<i32>().expect("Invalid data"))
        .collect::<Vec<_>>();

    println!("Calibration result: {}", calibrate(&input));
    println!(
        "First duplicate frequency: {}",
        first_duplicate_frequency(&input)
    );
}
