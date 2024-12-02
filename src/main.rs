use aoc_2024::days;
use std::fs;

fn main() {
    // will generalize this later
    let path = "data/day1";
    let data = fs::read_to_string(path).expect("failed to read file");
    let result = days::day1::run(&data);
    println!("{:?}", result);
}
