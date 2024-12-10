use std::fs;
use std::io::{BufRead, BufReader};

// allow the struct to be cloned
#[derive(Clone)]
struct Data {
    left_column: Vec<i32>,
    right_column: Vec<i32>,
}

fn read_file(file_path: &str) -> Data {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    // read the file line by line with a single buffer (does not load the whole file into memory)
    let file = fs::File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);
        left_column.push(parts[0].parse().unwrap());
        right_column.push(parts[1].parse().unwrap());
    }

    Data {
        left_column,
        right_column,
    }
}

fn solve_part_1(mut data: Data) -> i32 {
    data.left_column.sort();
    data.right_column.sort();
    
    assert_eq!(data.left_column.len(), data.right_column.len());
    let mut distance: i32 = 0;
    for i in 0..data.left_column.len() {
        let temp = data.left_column[i] - data.right_column[i];
        distance += temp.abs();
    }

    distance
}

fn main() {
    const EXPECTED_PART_1_EXAMPLE_RESULT: i32 = 11;
    let example_data = read_file("data/example.txt");
    let example_result = solve_part_1(example_data);
    assert_eq!(example_result, EXPECTED_PART_1_EXAMPLE_RESULT);

    let data = read_file("data/input.txt");
    let result = solve_part_1(data);
    println!("Part 1: {}", result);
}
