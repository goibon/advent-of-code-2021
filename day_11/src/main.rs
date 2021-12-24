use std::{collections::HashMap, env};

fn split_input(string: &str) -> Vec<Vec<u32>> {
    string
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|row| {
            row.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn get_adjacent_indices(
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_indices: Vec<(usize, usize)> = Vec::new();

    if row > 0 {
        adjacent_indices.push((row - 1, column));

        if column > 0 {
            adjacent_indices.push((row - 1, column - 1));
        }
        if column < max_column {
            adjacent_indices.push((row - 1, column + 1));
        }
    };
    if row < max_row {
        adjacent_indices.push((row + 1, column));

        if column > 0 {
            adjacent_indices.push((row + 1, column - 1));
        }
        if column < max_column {
            adjacent_indices.push((row + 1, column + 1));
        }
    };

    if column > 0 {
        adjacent_indices.push((row, column - 1));
    };
    if column < max_column {
        adjacent_indices.push((row, column + 1));
    };

    adjacent_indices
}

fn increment_all_entries(input: &mut Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut octopuses_about_to_flash: Vec<(usize, usize)> = Vec::new();
    for (row_index, row) in input.iter_mut().enumerate() {
        for (column_index, column) in row.iter_mut().enumerate() {
            *column += 1;
            if *column > 9 {
                octopuses_about_to_flash.push((row_index, column_index));
            }
        }
    }
    octopuses_about_to_flash
}

fn reset_flashed_entries(input: &mut Vec<Vec<u32>>) {
    for row in input.iter_mut() {
        for column in row.iter_mut() {
            if *column > 9 {
                *column = 0;
            }
        }
    }
}

fn flash_recursive(
    point: (usize, usize),
    already_flashed_map: &mut HashMap<(usize, usize), bool>,
    values: &mut Vec<Vec<u32>>,
) -> u32 {
    if let Some(has_flashed) = already_flashed_map.get_mut(&(point.0, point.1)) {
        if !*has_flashed && values[point.0][point.1] > 9 {
            *has_flashed = true;
            values[point.0][point.1] += 1;

            let neighbors_to_flash: Vec<(usize, usize)> =
                get_adjacent_indices(point.0, point.1, values.len() - 1, values[0].len() - 1)
                    .iter()
                    .filter_map(|(row, column)| {
                        if let Some(has_flashed) = already_flashed_map.get(&(*row, *column)) {
                            if *has_flashed {
                                None
                            } else {
                                Some((*row, *column))
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

            let mut amount_of_flashes: u32 = 1;
            for neighbor in neighbors_to_flash {
                values[neighbor.0][neighbor.1] += 1;
                amount_of_flashes +=
                    flash_recursive((neighbor.0, neighbor.1), already_flashed_map, values);
            }
            amount_of_flashes
        } else {
            0
        }
    } else {
        0
    }
}

fn part_1(input: &[Vec<u32>]) {
    let mut input = input.to_vec();
    let mut amount_of_flashes: u32 = 0;

    for _ in 1..=100 {
        let mut already_flashed_map: HashMap<(usize, usize), bool> = HashMap::new();
        for (row_index, row) in input.iter().enumerate() {
            for (column_index, _) in row.iter().enumerate() {
                already_flashed_map.insert((row_index, column_index), false);
            }
        }
        let octopuses_about_to_flash = increment_all_entries(&mut input);
        for octopus in octopuses_about_to_flash {
            amount_of_flashes += flash_recursive(octopus, &mut already_flashed_map, &mut input);
        }
        reset_flashed_entries(&mut input);
    }
    println!("Part 1: {:?}", amount_of_flashes);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = std::fs::read_to_string(path).expect("Error reading file.");
    let input = split_input(&input);
    part_1(&input);
}
