use std::env;

fn split_input(string: &String) -> Vec<Vec<u32>> {
    string
        .split("\n")
        .filter(|token| !token.is_empty())
        .map(|token| {
            token
                .chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn get_adjacent_indices(
    column: usize,
    row: usize,
    max_column: usize,
    max_row: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_indices: Vec<(usize, usize)> = Vec::new();

    if column > 0 {
        adjacent_indices.push((column - 1, row))
    };
    if column < max_column {
        adjacent_indices.push((column + 1, row))
    };
    if row > 0 {
        adjacent_indices.push((column, row - 1))
    };
    if row < max_row {
        adjacent_indices.push((column, row + 1))
    };

    adjacent_indices
}

fn is_point_lower_than_orthogonal_neighbors(
    point_row: usize,
    point_column: usize,
    map: &Vec<Vec<u32>>,
) -> bool {
    let point_value = map[point_row][point_column];
    let mut is_lower_than_all_adjacent_points = true;
    for (column, row) in get_adjacent_indices(
        point_column,
        point_row,
        map[point_row].len() - 1,
        map.len() - 1,
    ) {
        let adjacent_value = map[row][column];
        if adjacent_value <= point_value {
            is_lower_than_all_adjacent_points = false;
            break;
        }
    }

    is_lower_than_all_adjacent_points
}

struct LowPoint {
    row: usize,
    column: usize,
    value: u32,
}

fn find_all_low_points(input: &Vec<Vec<u32>>) -> Vec<LowPoint> {
    let mut low_points = Vec::new();
    for row_index in 0..input.len() {
        for column_index in 0..input[row_index].len() {
            if is_point_lower_than_orthogonal_neighbors(row_index, column_index, input) {
                let low_point = input[row_index][column_index];
                low_points.push(LowPoint {
                    row: row_index,
                    column: column_index,
                    value: low_point,
                });
            }
        }
    }
    low_points
}

fn part_1(input: &Vec<Vec<u32>>) {
    let mut result: u32 = 0;
    for low_point in find_all_low_points(input) {
        result += low_point.value + 1;
    }

    println!("Part 1: {:?}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    part_1(&input);
}
