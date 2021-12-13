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

fn identify_number_by_unique_char_count<'a>(digits: &'a [&str], char_count: usize) -> &'a str {
    digits
        .iter()
        .find(|number| number.chars().count() == char_count)
        .unwrap()
}

fn get_number_with_chars_and_length<'a>(
    required_chars: &'a [char],
    required_length: usize,
    digits_to_search: &'a [&str],
) -> (&'a str, char) {
    let number = digits_to_search
        .iter()
        .find(|digit| {
            digit.chars().count() == required_length
                && required_chars
                    .iter()
                    .all(|required_char| digit.contains(*required_char))
        })
        .unwrap();
    let new_char = get_unique_char(required_chars, &number.chars().collect::<Vec<_>>());
    (number, new_char)
}

fn get_unique_char(known_chars: &[char], iter_with_unknown_char: &[char]) -> char {
    *iter_with_unknown_char
        .iter()
        .find(|&char| !known_chars.iter().any(|known_char| char == known_char))
        .unwrap()
}

fn get_unique_char_from_strings(known_chars: &str, string_with_unknown_char: &str) -> char {
    get_unique_char(
        &known_chars.chars().collect::<Vec<_>>(),
        &string_with_unknown_char.chars().collect::<Vec<_>>(),
    )
}

fn compare_letters(left: &str, right: &str) -> bool {
    left.chars().count() == right.chars().count()
        && left.chars().all(|left_char| right.contains(left_char))
}

struct Digit {
    number: u32,
    string: String,
}

fn part_2(input: &Vec<(Vec<&str>, Vec<&str>)>) {
    let mut result: u32 = 0;
    for (numbers, output_digits) in input.iter() {
        let one = identify_number_by_unique_char_count(&numbers, 2);
        let four = identify_number_by_unique_char_count(&numbers, 4);
        let seven = identify_number_by_unique_char_count(&numbers, 3);
        let eight = identify_number_by_unique_char_count(&numbers, 7);

        let top_segment = get_unique_char_from_strings(one, seven);

        let mut required_chars_for_nine = four.chars().collect::<Vec<_>>();
        required_chars_for_nine.push(top_segment);
        let (nine, bottom_segment) =
            get_number_with_chars_and_length(&required_chars_for_nine, 6, &numbers);
        let bottom_left_segment = get_unique_char_from_strings(&nine, &eight);

        let mut required_chars_for_three = one.chars().collect::<Vec<_>>();
        required_chars_for_three.extend(vec![top_segment, bottom_segment]);
        let (three, middle_segment) =
            get_number_with_chars_and_length(&required_chars_for_three, 5, &numbers);

        let mut known_chars = one.chars().collect::<Vec<_>>();
        known_chars.push(middle_segment);
        let top_left_segment = get_unique_char(&known_chars, &four.chars().collect::<Vec<_>>());

        let eight_without_middle = eight
            .chars()
            .filter(|&char| char != middle_segment)
            .collect::<Vec<_>>();
        let zero = eight_without_middle.iter().cloned().collect::<String>();

        let six = *numbers
            .iter()
            .find(|&number| {
                number.chars().count() == 6
                    && !compare_letters(number, &zero)
                    && !compare_letters(number, &nine)
            })
            .unwrap();
        let bottom_right_segment = get_unique_char(
            &vec![
                top_segment,
                top_left_segment,
                middle_segment,
                bottom_left_segment,
                bottom_segment,
            ],
            &six.chars().collect::<Vec<_>>(),
        );

        let top_right_segment = get_unique_char(
            &vec![bottom_right_segment],
            &one.chars().collect::<Vec<_>>(),
        );
        let two = String::from_iter(vec![
            top_segment,
            top_right_segment,
            middle_segment,
            bottom_left_segment,
            bottom_segment,
        ]);
        let five = String::from_iter(vec![
            top_segment,
            top_left_segment,
            middle_segment,
            bottom_right_segment,
            bottom_segment,
        ]);

        let all_numbers = vec![
            Digit {
                number: 0,
                string: zero,
            },
            Digit {
                number: 1,
                string: one.to_string(),
            },
            Digit {
                number: 2,
                string: two,
            },
            Digit {
                number: 3,
                string: three.to_string(),
            },
            Digit {
                number: 4,
                string: four.to_string(),
            },
            Digit {
                number: 5,
                string: five,
            },
            Digit {
                number: 6,
                string: six.to_string(),
            },
            Digit {
                number: 7,
                string: seven.to_string(),
            },
            Digit {
                number: 8,
                string: eight.to_string(),
            },
            Digit {
                number: 9,
                string: nine.to_string(),
            },
        ];

        let mut output_string: String = String::new();
        for digit in output_digits {
            if let Some(identified_digit) = all_numbers
                .iter()
                .find(|number| compare_letters(digit, &number.string))
            {
                output_string.push_str(&identified_digit.number.to_string());
            };
        }
        let parsed_output_string: u32 = output_string.parse().unwrap();
        result += parsed_output_string;
    }

    println!("Part 2: {:?}", result);
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
    part_2(&input);
}
