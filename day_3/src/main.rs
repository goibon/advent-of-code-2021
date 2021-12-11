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

fn create_column_vector_for_specific_index(rows: &Vec<&str>, index: usize) -> Vec<u32> {
    let mut column: Vec<u32> = Vec::new();

    for &row in rows {
        let char_at_index = row.chars().skip(index).take(1).collect::<Vec<char>>()[0];
        if let Some(digit) = char_at_index.to_digit(2) {
            column.push(digit);
        }
    }

    column
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

fn get_rating(input: &Vec<&str>, default_bit: char, use_least_common_bit: bool) -> u32 {
    let mut remaining_numbers = input.clone();
    let mut index: usize = 0;
    while remaining_numbers.len() > 1 {
        let column = create_column_vector_for_specific_index(&remaining_numbers, index);
        let mut most_common_bit = get_most_common_bit(&column);

        most_common_bit = if most_common_bit == '-' {
            default_bit
        } else if use_least_common_bit {
            if most_common_bit == '1' {
                '0'
            } else {
                '1'
            }
        } else {
            most_common_bit
        };

        let mut new_remaining_numbers: Vec<&str> = Vec::new();
        for number in remaining_numbers {
            if let Some(char_at_index) = number.chars().skip(index).next() {
                if char_at_index == most_common_bit {
                    new_remaining_numbers.push(number);
                }
            }
        }
        remaining_numbers = new_remaining_numbers;
        index += 1;
    }

    if let Ok(number) = usize::from_str_radix(remaining_numbers[0], 2) {
        number as u32
    } else {
        0
    }
}

fn solve_part_2(input: &Vec<&str>) {
    let oxygen_generator_rating = get_rating(input, '1', false);
    let co2_scrubber_rating = get_rating(input, '0', true);
    println!(
        "Part 2: {:?}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    solve_part_1(&input);
    solve_part_2(&input);
}
