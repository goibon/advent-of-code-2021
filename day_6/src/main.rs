use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn split_input(string: &String) -> Vec<u32> {
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

fn convert_iterator_to_map(input: &[u32]) -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for &number in input.iter() {
        map.entry(number)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }
    map
}

fn part_1(input: &[u32]) {
    let mut map = convert_iterator_to_map(input);
    let mut result: u32 = 0;
    for day in 0..80 {
        let mut new_map: HashMap<u32, u32> = HashMap::new();
        for (key, &value) in map.iter().sorted() {
            match key {
                0 => {
                    new_map
                        .entry(6)
                        .and_modify(|entry| *entry += value)
                        .or_insert(value);
                    new_map.insert(8, value);
                }
                &_ => {
                    let new_key = key - 1;
                    new_map
                        .entry(new_key)
                        .and_modify(|entry| *entry += value)
                        .or_insert(value);
                }
            }
        }

        if day == 79 {
            result = new_map.values().sum();
        }
        map = new_map;
    }

    println!("Part 1: {:?}", result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    part_1(&input);
}
