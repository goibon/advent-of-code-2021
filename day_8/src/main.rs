use std::env;

fn split_input(string: &String) -> Vec<(Vec<&str>, Vec<&str>)> {
    string
        .split("\n")
        .filter(|token| !token.is_empty())
        .map(|line| {
            let split_line: Vec<&str> = line.split(" | ").collect();
            let left_side = split_line[0].split(" ").collect();
            let right_side = split_line[1].split(" ").collect();
            (left_side, right_side)
        })
        .collect()
}

fn part_1(input: &Vec<&str>) {
    let mut count: u32 = 0;
    for &digit in input.iter() {
        match digit.chars().count() {
            2 | 3 | 4 | 7 => {
                count += 1;
            }
            _ => {}
        }
    }

    println!("Part 1: {:?}", count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    let flattened_vector: Vec<&str> = input
        .iter()
        .cloned()
        .flat_map(|(_, output_digits)| output_digits)
        .collect();
    part_1(&flattened_vector);
}
