use std::fs;

fn read_file(file_path: &str) -> Vec<Vec<isize>> {
    let mut data: Vec<Vec<isize>> = Vec::new();
    let contents = fs::read_to_string(file_path)
        .expect(&format!("Failed to read file: {}", file_path));

    for line in contents.lines() {
        let mut row: Vec<isize> = Vec::new();
        for number in line.split_whitespace() {
            row.push(number.parse().unwrap());
        }
        data.push(row);
    }

    data
}

fn is_safe(report: &Vec<isize>) -> bool {
    // need to check the following conditions:
    // - sorted or reverse sorted
    // - the absolute difference between each adjacent pair of numbers is between 1 and 3
    let is_sorted = report.is_sorted_by(|a, b| a<=b) || report.is_sorted_by(|a, b| a>=b);
    let distances_ok = report.windows(2).all(|pair| {
        let diff = (pair[1]).abs_diff(pair[0]);
        diff >= 1 && diff <= 3
    });

    is_sorted && distances_ok
}

fn is_dampended_safe(report: &Vec<isize>) -> bool {
    // check each condition from `is_safe` while allowing a single item to be removed
    //first check if the report is safe
    if is_safe(report) {
        return true;
    }
    
    // for each index in the report, create a copy without it and check if it is safe
    for i in 0..report.len() {
        let mut copy = report.clone();
        copy.remove(i);
        if is_safe(&copy) {
            return true;
        }
    }

    false
}

fn solve_part_1(data: &Vec<Vec<isize>>) -> isize {
    data.iter().filter(|report| is_safe(report)).count() as isize
}

fn solve_part_2(data: &Vec<Vec<isize>>) -> isize {
    // from each unsafe report, 1 value is allowed to be removed if the resulting report is safe
    data.iter().filter(|report| is_dampended_safe(report)).count() as isize
}

fn main() {
    let example_data = read_file("data/example.txt");
    let data = read_file("data/input.txt");

    const EXPECTED_PART_1_EXAMPLE_RESULT: isize = 2;
    let example_part_1_result = solve_part_1(&example_data);
    assert_eq!(example_part_1_result, EXPECTED_PART_1_EXAMPLE_RESULT);

    let part_1_result = solve_part_1(&data);
    println!("Part 1: {}", part_1_result);

    const EXPECTED_PART_2_EXAMPLE_RESULT: isize = 4;
    let example_part_2_result = solve_part_2(&example_data);
    assert_eq!(example_part_2_result, EXPECTED_PART_2_EXAMPLE_RESULT);

    let part_2_result = solve_part_2(&data);
    println!("Part 2: {}", part_2_result);
}
