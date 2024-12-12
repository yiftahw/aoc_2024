use std::{fs, isize};
use regex::Regex;

const PAIR_PREFIX: &str = "mul(";
const PAIR_SUFFIX: &str = ")";

fn part_1(data: &String) -> isize {
    let re = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();
    re.find_iter(&data).map(|s| {
        let pair = &data[s.start() + PAIR_PREFIX.len()..s.end() - PAIR_SUFFIX.len()];
        let mut nums: Vec<&str> = pair.split(",").collect();
        let num1 = nums.pop().unwrap().parse::<isize>().unwrap();
        let num2 = nums.pop().unwrap().parse::<isize>().unwrap();
        num1 * num2
    }).sum::<isize>()
}

fn main() {
    let example_data = fs::read_to_string("data/example.txt").expect("Failed to read example file");
    let data: String = fs::read_to_string("data/input.txt").expect("Failed to read input file");

    const EXAMPLE_RESULT: isize = 161;
    let result = part_1(&example_data);
    assert_eq!(result, EXAMPLE_RESULT);

    println!("Part 1: {}", part_1(&data));
}
