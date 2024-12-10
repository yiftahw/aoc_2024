use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

// allow the struct to be cloned (was not used in the end)
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

fn solve_part_1(data: &mut Data) -> i32 {
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

fn solve_part_2(data: &mut Data) -> i32 {
    // side effect of part 1 is that the columns are sorted
    // we need to create a histogram of the right column
    assert!(data.right_column.is_sorted());
    let mut histogram: HashMap<i32, i32> = HashMap::new();
    
    // start condition
    let mut count: i32 = 0; 
    let mut current_value: i32 = data.right_column[0];

    for &value in data.right_column.iter() {
        if value == current_value {
            count += 1;
        } else {
            histogram.insert(current_value, count);
            current_value = value;
            count = 1;
        }
    }

    // insert the last value
    histogram.insert(current_value, count);

    // for each value in the left column, we multiply it by the histogram value
    let mut sum: i32 = 0;
    for &value in data.left_column.iter() {
        let count: i32 = *histogram.get(&value).unwrap_or(&0);
        sum += value * count;
    }

    sum
}

fn main() {
    let mut example_data = read_file("data/example.txt");
    let mut data = read_file("data/input.txt");

    const EXPECTED_PART_1_EXAMPLE_RESULT: i32 = 11;
    let example_result = solve_part_1(&mut example_data);
    assert_eq!(example_result, EXPECTED_PART_1_EXAMPLE_RESULT);

    let result = solve_part_1(&mut data);
    println!("Part 1: {}", result);

    // side effect, both vectors in `example_data` and `data` are sorted

    const EXPECTED_PART_2_EXAMPLE_RESULT: i32 = 31;
    let example_result = solve_part_2(&mut example_data);
    assert_eq!(example_result, EXPECTED_PART_2_EXAMPLE_RESULT);

    let result = solve_part_2(&mut data);
    println!("Part 2: {}", result);
}
