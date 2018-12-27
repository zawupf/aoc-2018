extern crate aoc_2018;
use aoc_2018::d06::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/06/input.txt").expect("Missing input data");
    let input = content
        .trim_end()
        .split('\n')
        .map(|data| data.parse().unwrap())
        .collect::<Vec<_>>();

    println!("Largest area: {}", largest_area(&input));
    println!("Region size: {}", region_size(&input, 10000));
}
