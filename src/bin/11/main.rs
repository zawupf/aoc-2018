extern crate aoc_2018;
use aoc_2018::d11::*;
use aoc_2018::read_file;

fn main() {
    let content = read_file("src/bin/11/input.txt").expect("Missing input data");
    let input = content.trim_end().parse::<i32>().unwrap();

    let (x, y) = largest_total_power_coords(input, 3).0;
    println!("Square with largest total power: {},{}", x, y);
    let (x, y, s) = largest_total_power_coords_any_size(input).0;
    println!("Any square with largest total power: {},{},{}", x, y, s);
}
