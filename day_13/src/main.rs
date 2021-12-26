use std::{collections::HashMap, env};

const FOLD_EXPRESSION: &str = "fold along ";

fn split_input(string: &str) -> (Vec<(u32, u32)>, Vec<(char, u32)>) {
    let lines: Vec<&str> = string.split('\n').filter(|line| !line.is_empty()).collect();
    let first_fold_instruction_index = lines
        .iter()
        .position(|line| line.contains(FOLD_EXPRESSION))
        .unwrap();
    let (dot_coordinates, fold_instructions) = lines.split_at(first_fold_instruction_index);

    let dot_coordinates: Vec<(u32, u32)> = dot_coordinates
        .iter()
        .map(|coordinate_string| {
            let split_line: Vec<u32> = coordinate_string
                .split(',')
                .filter_map(|coordinate| coordinate.parse().ok())
                .collect();
            (split_line[0], split_line[1])
        })
        .collect();

    let fold_instructions: Vec<(char, u32)> = fold_instructions
        .iter()
        .map(|fold_string| {
            let split_line: Vec<&str> = fold_string.split('=').collect();
            (
                split_line[0].chars().last().unwrap(),
                split_line[1].parse().unwrap(),
            )
        })
        .collect();

    (dot_coordinates, fold_instructions)
}

const FOLD_UP_INSTRUCTION: char = 'y';

fn get_flipped_index(index_to_flip: u32, middle: u32) -> u32 {
    middle - (index_to_flip - middle)
}

fn fold_up(dot_map: &HashMap<(u32, u32), bool>, folding_index: u32) -> HashMap<(u32, u32), bool> {
    let mut new_dot_map: HashMap<(u32, u32), bool> = HashMap::new();
    for ((column, row), dot) in dot_map {
        if *row < folding_index {
            new_dot_map.entry((*column, *row)).or_insert(*dot);
        } else {
            let new_row = get_flipped_index(*row, folding_index);
            new_dot_map.entry((*column, new_row)).or_insert(*dot);
        }
    }
    new_dot_map
}

fn fold_left(dot_map: &HashMap<(u32, u32), bool>, folding_index: u32) -> HashMap<(u32, u32), bool> {
    let mut new_dot_map: HashMap<(u32, u32), bool> = HashMap::new();
    for ((column, row), dot) in dot_map {
        if *column < folding_index {
            new_dot_map.entry((*column, *row)).or_insert(*dot);
        } else {
            let new_column = get_flipped_index(*column, folding_index);
            new_dot_map.entry((new_column, *row)).or_insert(*dot);
        }
    }
    new_dot_map
}

fn coordinates_to_map(dot_coordinates: &[(u32, u32)]) -> HashMap<(u32, u32), bool> {
    let mut map: HashMap<(u32, u32), bool> = HashMap::new();

    for (column, row) in dot_coordinates {
        map.insert((*column, *row), true);
    }

    map
}

fn part_1(dot_coordinates: &[(u32, u32)], fold_instructions: &[(char, u32)]) {
    let map = coordinates_to_map(dot_coordinates);

    if let Some((fold_instruction, folding_index)) = fold_instructions.first() {
        let result_of_folding = if *fold_instruction == FOLD_UP_INSTRUCTION {
            fold_up(&map, *folding_index)
        } else {
            fold_left(&map, *folding_index)
        };
        println!("Part 1: {:?}", result_of_folding.len());
    }
}

const DOT_CHARACTER: char = '#';
const EMPTY_FIELD_CHARACTER: char = '.';

fn print_map(dot_map: &HashMap<(u32, u32), bool>) {
    let mut rows: Vec<u32> = Vec::new();
    let mut columns: Vec<u32> = Vec::new();

    for (column, row) in dot_map.keys() {
        rows.push(*row);
        columns.push(*column);
    }

    rows.sort_unstable();
    columns.sort_unstable();

    for row in 0..=rows[rows.len() - 1] {
        let mut row_string = "".to_string();
        for column in 0..=columns[columns.len() - 1] {
            match dot_map.get(&(column, row)) {
                Some(true) => {
                    row_string += &DOT_CHARACTER.to_string();
                }
                None | Some(false) => {
                    row_string += &EMPTY_FIELD_CHARACTER.to_string();
                }
            }
        }
        println!("{}", row_string);
    }
}

fn part_2(dot_coordinates: &[(u32, u32)], fold_instructions: &[(char, u32)]) {
    let mut map = coordinates_to_map(dot_coordinates);
    for (fold_instruction, folding_index) in fold_instructions {
        map = if *fold_instruction == FOLD_UP_INSTRUCTION {
            fold_up(&map, *folding_index)
        } else {
            fold_left(&map, *folding_index)
        };
    }
    print_map(&map);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = std::fs::read_to_string(path).expect("Error reading file.");
    let (dot_coordinates, fold_instructions) = split_input(&input);

    part_1(&dot_coordinates, &fold_instructions);
    part_2(&dot_coordinates, &fold_instructions);
}
