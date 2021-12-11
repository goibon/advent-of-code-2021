use std::env;

fn split_input(string: &String) -> Vec<&str> {
    string
        .split("\n")
        .filter(|token| !token.is_empty())
        .collect()
}

fn create_column_vector(rows: &Vec<&str>) -> Vec<Vec<u32>> {
    let mut columns: Vec<Vec<u32>> = Vec::new();

    for _ in 0..rows[0].len() {
        columns.push(Vec::new());
    }

    for &row in rows {
        for (index, value) in row.chars().enumerate() {
            columns[index].push(value.to_digit(2).unwrap())
        }
    }

    columns
}

fn get_most_common_bit(input: &Vec<u32>) -> char {
    let mut zero_count: u32 = 0;
    let mut one_count: u32 = 0;

    for number in input {
        match number {
            0 => {
                zero_count += 1;
            }
            1 => {
                one_count += 1;
            }
            &_ => break,
        }

        if zero_count > input.len() as u32 / 2 {
            return '0';
        } else if one_count > input.len() as u32 / 2 {
            return '1';
        }
    }

    '-'
}

fn solve_part_1(input: &Vec<&str>) {
    let columns = create_column_vector(&input);
    let mut gamma_rate = String::new();

    for column in columns {
        let most_common_bit = get_most_common_bit(&column);
        gamma_rate.push(most_common_bit);
    }

    let mut flipper_bits = String::new();
    for _ in 0..gamma_rate.len() {
        flipper_bits.push('1');
    }

    if let Ok(gamma_rate) = usize::from_str_radix(&gamma_rate, 2) {
        if let Ok(flipper_bits) = usize::from_str_radix(&flipper_bits, 2) {
            let epsilon_rate = gamma_rate ^ flipper_bits;
            let part_1 = gamma_rate * epsilon_rate;
            println!("Part 1: {:?}", part_1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    solve_part_1(&input);
}
