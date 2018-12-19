extern crate aoc_2018;
use aoc_2018::d02::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/02/input.txt").expect("Missing input data");
    let input = content.split_whitespace().collect::<Vec<_>>();

    println!("Check-sum: {}", checksum(&input));
    println!(
        "Common letters: {}",
        common_letters(&input).unwrap_or_else(|| "None".to_owned())
    );
}
