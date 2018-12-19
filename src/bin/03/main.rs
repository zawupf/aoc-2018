extern crate aoc_2018;
use aoc_2018::d03::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/03/input.txt").expect("Missing input data");
    let input = content
        .trim_end()
        .split('\n')
        .map(|data| data.parse().unwrap())
        .collect::<Vec<_>>();

    println!("Claims: {}", input.len());

    println!(
        "Double claimed square inches: {}",
        double_claimed_square_inches(&input)
    );
    println!(
        "Non overlapping claim: {}",
        find_first_valid_claim_id(&input).unwrap()
    );
}
