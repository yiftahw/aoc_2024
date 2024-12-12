use std::{fs, isize};
use regex::Regex;

const PAIR_PREFIX: &str = "mul(";
const PAIR_SUFFIX: &str = ")";
const DO: &str = "do()";
const DONT: &str = "don't()";

fn parse_pair_and_multiply(pair: &str) -> isize {
    let mut nums: Vec<&str> = pair.split(",").collect();
    let num1 = nums.pop().unwrap().parse::<isize>().unwrap();
    let num2 = nums.pop().unwrap().parse::<isize>().unwrap();
    num1 * num2
}

fn part_1(data: &String) -> isize {
    let re = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();
    re.find_iter(&data).map(|s| {
        let pair = &data[s.start() + PAIR_PREFIX.len()..s.end() - PAIR_SUFFIX.len()];
        parse_pair_and_multiply(pair)
    }).sum::<isize>()
}

fn part_2(data: &String) -> isize {
    let re = Regex::new(r"mul\([0-9]+\,[0-9]+\)|do\(\)|don\'t\(\)").unwrap();
    let mut is_do: bool = true;
    re.find_iter(&data).map(|s| {
        let sub_str = &data[s.start()..s.end()];
        match sub_str {
            DO => {
                is_do = true;
                0
            }
            DONT => {
                is_do = false;
                0
            }
            _ if is_do => {
                let pair = &sub_str[PAIR_PREFIX.len()..sub_str.len() - PAIR_SUFFIX.len()];
                parse_pair_and_multiply(pair)
            }
            _ => 0
        }
    }).sum::<isize>()
}

fn main() {
    let data: String = fs::read_to_string("data/input.txt").expect("Failed to read input file");
    let example_data = fs::read_to_string("data/example.txt").expect("Failed to read example file");
    let example_data_2 = fs::read_to_string("data/example_part_2.txt").expect("Failed to read example file 2");

    const EXAMPLE_PART_1_RESULT: isize = 161;
    let result = part_1(&example_data);
    assert_eq!(result, EXAMPLE_PART_1_RESULT);

    println!("Part 1: {}", part_1(&data));

    const EXAMPLE_PART_2_RESULT: isize = 48;
    let result = part_2(&example_data_2);
    assert_eq!(result, EXAMPLE_PART_2_RESULT);

    println!("Part 2: {}", part_2(&data));
}
