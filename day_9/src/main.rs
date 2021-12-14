use std::collections::HashMap;
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

fn get_adjacent_indices_higher_in_value(
    row: usize,
    column: usize,
    map: &Vec<Vec<u32>>,
) -> Vec<(usize, usize)> {
    let value = map[row][column];
    get_adjacent_indices(column, row, map[row].len() - 1, map.len() - 1)
        .iter()
        .cloned()
        .filter(|neighbor| map[neighbor.1][neighbor.0] > value)
        .collect()
}

fn get_basin_members(
    input: &Vec<Vec<u32>>,
    row: usize,
    column: usize,
) -> HashMap<(usize, usize), u32> {
    let value = input[row][column];
    let higher_value_neighbors: Vec<(usize, usize)> =
        get_adjacent_indices_higher_in_value(row, column, input)
            .iter()
            .cloned()
            .filter(|(neighbor_column, neighbor_row)| input[*neighbor_row][*neighbor_column] < 9)
            .collect();
    if higher_value_neighbors.is_empty() {
        HashMap::from([((row, column), value)])
    } else {
        let mut partial_basin: HashMap<(usize, usize), u32> =
            HashMap::from([((row, column), value)]);
        for (neighbor_column, neighbor_row) in higher_value_neighbors {
            partial_basin.extend(get_basin_members(input, neighbor_row, neighbor_column));
        }
        partial_basin
    }
}

fn part_2(input: &Vec<Vec<u32>>) {
    let mut basins: Vec<usize> = Vec::new();
    for low_point in find_all_low_points(input) {
        let basin = get_basin_members(input, low_point.row, low_point.column);
        basins.push(basin.len());
    }
    basins.sort();
    let product_of_top_3_largest_basins: usize =
        basins.iter().cloned().skip(basins.len() - 3).product();

    println!("Part 2: {:?}", product_of_top_3_largest_basins);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    part_1(&input);
    part_2(&input);
}
