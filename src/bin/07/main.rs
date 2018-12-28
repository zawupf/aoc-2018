extern crate aoc_2018;
use aoc_2018::d07::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/07/input.txt").expect("Missing input data");
    let input = content
        .trim_end()
        .split('\n')
        .collect::<Vec<_>>();

    println!("Task order: {}", task_execution_order(&input));
    println!("Task duration: {}", task_duration(&input, 5, 60));
}
