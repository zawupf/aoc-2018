extern crate aoc_2018;
use aoc_2018::d04::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/04/input.txt").expect("Missing input data");
    let input = content.trim_end().split('\n').collect::<Vec<_>>();

    let guards_data = read_data(&input);
    let guards: Vec<&Guard> = guards_data.values().collect();

    println!("Strategy 1: {}", strategy_1(&guards));
    println!("Strategy 2: {}", strategy_2(&guards));
}
