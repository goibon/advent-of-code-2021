use std::{collections::HashMap, env};

fn split_input(string: &str) -> (&str, HashMap<&str, char>) {
    let mut split_lines = string.split('\n').filter(|line| !line.is_empty());
    let template: &str = split_lines.next().unwrap();
    let mut insertion_rules: HashMap<&str, char> = HashMap::new();

    for line in split_lines {
        let split_line: Vec<&str> = line.split(" -> ").collect();
        insertion_rules
            .entry(split_line[0])
            .or_insert_with(|| split_line[1].chars().last().unwrap());
    }

    (template, insertion_rules)
}

fn get_char_pairs_from_string(string: &str) -> Vec<String> {
    let mut char_pairs = Vec::new();
    let char_vector: Vec<char> = string.chars().collect();
    for index in (1..string.len()).rev() {
        let pair = char_vector[index - 1].to_string() + &char_vector[index].to_string();
        char_pairs.push(pair);
    }

    char_pairs
}

fn insert_into_template(template: &str, insertion_rules: &HashMap<&str, char>) -> String {
    let char_pairs = get_char_pairs_from_string(template);
    let mut new_template: Vec<char> = template.chars().collect();

    for (index, char_pair) in char_pairs.iter().enumerate() {
        if let Some(char_to_insert) = insertion_rules.get(char_pair.as_str()) {
            let insertion_index = template.len() - 1 - index;
            new_template.insert(insertion_index, *char_to_insert);
        }
    }

    new_template.into_iter().collect()
}

fn count_character_occurrences(string: &str) -> HashMap<char, u32> {
    let mut character_counts = HashMap::new();

    for character in string.chars().collect::<Vec<_>>() {
        *character_counts.entry(character).or_insert(0) += 1;
    }

    character_counts
}

fn part_1(template: &str, insertion_rules: &HashMap<&str, char>) {
    let mut template = template.to_string();
    for _ in 1..=10 {
        template = insert_into_template(&template, insertion_rules);
    }
    let character_counts = count_character_occurrences(&template);
    let mut counts: Vec<u32> = character_counts.values().cloned().collect();
    counts.sort_unstable();
    let least_frequent = counts.first().unwrap();
    let most_frequent = counts.last().unwrap();

    println!("Part 1: {}", most_frequent - least_frequent);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = std::fs::read_to_string(path).expect("Error reading file.");
    let (template, insertion_rules) = split_input(&input);
    part_1(template, &insertion_rules);
}
