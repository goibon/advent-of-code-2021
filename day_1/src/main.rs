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

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    let part_1 = count_increments(&input);
    println!("Part 1: {:?}", part_1);
}
