use aoc_2024::days;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u32 = args
        .get(1)
        .expect("please provide day as argument")
        .parse()
        .expect("day must be numeric");
    let path = format!("data/day{}", args[1]);
    let data = fs::read_to_string(path).expect("failed to read file");
    // tood generate this match statement with a macro
    let result = match day {
        1 => days::day1::run(&data),
        2 => days::day2::run(&data),
        3 => days::day3::run(&data),
        4 => days::day4::run(&data),
        5..=25 => todo!(),
        _ => panic!("invalid day"),
    };
    println!("{:?}", result);
}
