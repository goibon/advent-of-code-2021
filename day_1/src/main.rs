use std::env;

fn split_input(string: &String) -> Vec<u16> {
    string
        .split("\n")
        .filter_map(|token| token.parse().ok())
        .collect()
}

fn count_window_increments(input: &Vec<u16>, size: usize) -> u16 {
    let mut count: u16 = 0;
    let mut windows = input.windows(size);

    let mut previous_sum: u16 = 0;

    if let Some(first_window) = windows.next() {
        previous_sum = first_window.iter().sum();
    }

    while let Some(next_window) = windows.next() {
        let current_sum = next_window.iter().sum();
        if current_sum > previous_sum {
            count += 1;
        }

        previous_sum = current_sum;
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    let part_1 = count_window_increments(&input, 1);
    println!("Part 1: {:?}", part_1);

    let part_2 = count_window_increments(&input, 3);
    println!("Part 2: {:?}", part_2);
}
