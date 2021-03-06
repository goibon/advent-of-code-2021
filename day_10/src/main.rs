use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn split_input(string: &String) -> Vec<&str> {
    string
        .split("\n")
        .filter(|token| !token.is_empty())
        .collect()
}

const OPENING_CHARS: [char; 4] = ['(', '{', '[', '<'];
const CLOSING_CHARS: [char; 4] = [')', '}', ']', '>'];

fn find_first_illegal_character(input: &str) -> (Option<char>, HashMap<u8, char>) {
    let mut map: HashMap<u8, char> = HashMap::new();
    let mut current_depth: u8 = 0;
    for character in input.chars() {
        if OPENING_CHARS.contains(&character) {
            map.insert(current_depth, character);
            current_depth += 1;
            continue;
        }

        if current_depth == 0 {
            return (Some(character), map);
        }

        if let Some(current_opening_char) = map.get(&(current_depth - 1)) {
            if let Some(matching_closing_char) = find_matching_closing_char(current_opening_char) {
                if character == matching_closing_char {
                    map.remove(&(current_depth - 1));
                    current_depth -= 1;
                } else {
                    return (Some(character), map);
                }
            }
        }
    }

    (None, map)
}

fn find_matching_closing_char(opening_char: &char) -> Option<char> {
    OPENING_CHARS
        .iter()
        .position(|char| opening_char == char)
        .and_then(|index| CLOSING_CHARS.get(index))
        .copied()
}

fn part_1(input: &[&str]) {
    let scoring_map: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut score: u32 = 0;
    for line in input.iter() {
        if let (Some(first_illegal_char), _) = find_first_illegal_character(line) {
            score += scoring_map[&first_illegal_char];
        }
    }

    println!("Part 1: {:?}", score);
}

fn part_2(input: &[&str]) {
    let scoring_map: HashMap<char, u32> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut scores: Vec<u64> = Vec::new();
    for line in input.iter() {
        let mut score: u64 = 0;
        if let (None, map) = find_first_illegal_character(line) {
            map.iter().sorted().rev().for_each(|(_, value)| {
                if let Some(char_value) = find_matching_closing_char(value)
                    .and_then(|closing_char| scoring_map.get(&closing_char))
                {
                    score *= 5;
                    score += *char_value as u64;
                }
            });
            scores.push(score);
        }
    }

    scores.sort_unstable();

    println!(
        "Part 2: {:?}",
        scores
            .iter()
            .skip(scores.len() / 2)
            .collect::<Vec<_>>()
            .first()
            .unwrap()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    part_1(&input);
    part_2(&input);
}
