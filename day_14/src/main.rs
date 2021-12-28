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

fn get_char_pairs_from_string(string: &str) -> Vec<(char, char)> {
    let mut char_pairs = Vec::new();
    let char_vector: Vec<char> = string.chars().collect();
    for index in 1..string.len() {
        char_pairs.push((char_vector[index - 1], char_vector[index]))
    }

    char_pairs
}

fn recurse_insertion_pair(
    pair: &str,
    insertion_rules: &HashMap<&str, char>,
    depth: u32,
    cached_results: &mut HashMap<(String, u32), HashMap<char, u64>>,
) -> HashMap<char, u64> {
    if depth == 0 {
        let right_pair_char = pair.chars().collect::<Vec<_>>()[1];
        return HashMap::from([(right_pair_char, 1)]);
    }

    if let Some(cached_result) = cached_results.get(&(pair.to_string(), depth)) {
        return cached_result.clone();
    }

    let mut result = HashMap::new();
    if let Some(char_to_insert) = insertion_rules.get(pair) {
        let left_pair_char = pair.chars().collect::<Vec<_>>()[0];
        let right_pair_char = pair.chars().collect::<Vec<_>>()[1];
        let new_left_pair = vec![left_pair_char, *char_to_insert]
            .iter()
            .collect::<String>();
        let new_right_pair = vec![*char_to_insert, right_pair_char]
            .iter()
            .collect::<String>();

        result = recurse_insertion_pair(&new_left_pair, insertion_rules, depth - 1, cached_results);
        let second_count_map =
            recurse_insertion_pair(&new_right_pair, insertion_rules, depth - 1, cached_results);

        for key in second_count_map.keys() {
            *result.entry(*key).or_default() += *second_count_map.get(key).unwrap();
        }

        cached_results
            .entry((pair.to_owned(), depth))
            .or_insert_with(|| result.clone());
    }

    result
}

fn count_chars_after_insertions(
    template: &str,
    insertion_rules: &HashMap<&str, char>,
    depth: u32,
) -> u64 {
    let initial_pairs: Vec<Vec<char>> = get_char_pairs_from_string(template)
        .iter()
        .map(|(left, right)| vec![*left, *right])
        .collect();
    let mut result = String::new();
    result.extend(template.chars().collect::<Vec<_>>().first());

    let mut result_map: HashMap<char, u64> =
        HashMap::from([(*template.chars().collect::<Vec<_>>().first().unwrap(), 1)]);
    let mut cached_results: HashMap<(String, u32), HashMap<char, u64>> = HashMap::new();
    for pair in initial_pairs {
        let intermediate_result = recurse_insertion_pair(
            &pair.iter().collect::<String>(),
            insertion_rules,
            depth,
            &mut cached_results,
        );

        for key in intermediate_result.keys() {
            *result_map.entry(*key).or_default() += *intermediate_result.get(key).unwrap();
        }
    }

    let mut counts: Vec<_> = result_map.values().collect();
    counts.sort_unstable();
    let least_frequent = counts.first().unwrap();
    let most_frequent = counts.last().unwrap();
    *most_frequent - *least_frequent
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = std::fs::read_to_string(path).expect("Error reading file.");
    let (template, insertion_rules) = split_input(&input);

    let part_1 = count_chars_after_insertions(template, &insertion_rules, 10);
    println!("Part 1: {}", part_1);
    let part_2 = count_chars_after_insertions(template, &insertion_rules, 40);
    println!("Part 2: {}", part_2);
}
