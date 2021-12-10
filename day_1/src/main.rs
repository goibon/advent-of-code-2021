use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &String) -> Vec<u16> {
    string
        .split("\n")
        .filter_map(|token| token.parse().ok())
        .collect()
}

fn count_increments(input: &Vec<u16>) -> u16 {
    let mut count: u16 = 0;

    for index in 1..input.len() {
        if input[index] > input[index - 1] {
            count += 1;
        }
    }

    count
}

fn count_window_increments(input: &Vec<u16>) -> u16 {
    let mut count: u16 = 0;
    let mut windows = input.windows(3);

    let mut previous_sum: u16 = 0;

    if let Some(first_window) = windows.next() {
        previous_sum = first_window.iter().sum();
    }

    while let Some(next_window) = windows.next() {
        let current_sum = next_window.iter().sum();
        if current_sum > previous_sum {
            count += 1;
        }

        previous_sum = current_sum;
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    let part_1 = count_increments(&input);
    println!("Part 1: {:?}", part_1);

    let part_2 = count_window_increments(&input);
    println!("Part 2: {:?}", part_2);
}
