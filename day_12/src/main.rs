use std::{collections::HashMap, env};

fn split_input(string: &str) -> HashMap<String, Cave> {
    let cave_pairs: Vec<Vec<&str>> = string
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split('-').collect())
        .collect();
    let mut map: HashMap<String, Cave> = HashMap::new();
    for cave_pair in &cave_pairs {
        let left_cave_name = cave_pair[0];
        let right_cave_name = cave_pair[1];

        let left_cave = map
            .entry(left_cave_name.to_string())
            .or_insert_with(|| Cave::new(left_cave_name));
        if !left_cave
            .connections
            .iter()
            .any(|connection| connection == right_cave_name)
        {
            left_cave.connections.push(right_cave_name.to_string());
        }

        let right_cave = map
            .entry(right_cave_name.to_string())
            .or_insert_with(|| Cave::new(right_cave_name));
        if !right_cave
            .connections
            .iter()
            .any(|connection| connection == left_cave_name)
        {
            right_cave.connections.push(left_cave_name.to_string());
        }
    }

    map
}

#[derive(PartialEq)]
enum Size {
    Small,
    Big,
}

struct Cave {
    connections: Vec<String>,
    size: Size,
}

impl Cave {
    fn new(cave_name: &str) -> Cave {
        Cave {
            connections: Vec::new(),
            size: get_size(cave_name),
        }
    }
}

fn get_size(cave_name: &str) -> Size {
    if cave_name.chars().all(|character| character.is_uppercase()) {
        Size::Big
    } else {
        Size::Small
    }
}

const START_CAVE_NAME: &str = "start";
const END_CAVE_NAME: &str = "end";

fn recurse_caves(
    cave_name: &str,
    caves_visited: &str,
    caves: &HashMap<String, Cave>,
) -> Option<Vec<String>> {
    if cave_name == END_CAVE_NAME {
        return Some(vec![caves_visited.to_owned() + "," + cave_name]);
    }
    if cave_name == START_CAVE_NAME && !caves_visited.is_empty() {
        return None;
    }

    if let Some(cave) = caves.get(cave_name) {
        if cave.size == Size::Small && caves_visited.matches(cave_name).count() >= 1 {
            None
        } else {
            let mut visit_strings: Vec<String> = Vec::new();
            let caves_visited = if caves_visited.is_empty() {
                cave_name.to_string()
            } else {
                caves_visited.to_owned() + "," + cave_name
            };
            for connection_name in &cave.connections {
                if let Some(strings) = recurse_caves(connection_name, &caves_visited, caves) {
                    visit_strings.extend(strings);
                }
            }
            if visit_strings.is_empty() {
                None
            } else {
                Some(visit_strings)
            }
        }
    } else {
        None
    }
}

fn part_1(input: &HashMap<String, Cave>) {
    if let Some(paths) = recurse_caves(START_CAVE_NAME, "", input) {
        println!("Part 1: {:?}", paths.len());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = std::fs::read_to_string(path).expect("Error reading file.");
    let input = split_input(&input);
    part_1(&input);
}
