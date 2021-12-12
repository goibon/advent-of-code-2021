use std::env;

fn split_input(string: &String) -> Vec<i32> {
    string
        .split("\n")
        .filter(|token| !token.is_empty())
        .collect::<Vec<&str>>()
        .first()
        .map_or_else(|| "", |string| string)
        .split(",")
        .filter_map(|string| string.parse().ok())
        .collect()
}

fn part_1(input: &[i32]) {
    let mut mutable_list: Vec<i32> = input.iter().cloned().collect();
    mutable_list.sort();

    let middle_value_index = ((mutable_list.len() as f64 - 1.0) / 2.0).floor() as usize;
    let median: i32;
    if mutable_list.len() % 2 == 0 {
        median = ((mutable_list[middle_value_index] + mutable_list[middle_value_index + 1]) as f64
            / 2.0)
            .floor() as i32;
    } else {
        median = mutable_list[middle_value_index];
    }

    let mut total_fuel_used: u32 = 0;
    for value in mutable_list.iter() {
        total_fuel_used += (value - median).abs() as u32;
    }

    println!("Part 1: {:?}", total_fuel_used);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    part_1(&input);
}
