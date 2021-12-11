use std::collections::HashMap;
use std::env;

fn split_input(string: &String) -> Vec<&str> {
    string
        .split("\n")
        .filter(|token| !token.is_empty())
        .collect()
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

struct Line {
    start: Coordinate,
    end: Coordinate,
}

impl Line {
    fn new(input: &str) -> Line {
        let split: Vec<Vec<u32>> = input
            .split(" -> ")
            .map(|token| {
                token
                    .split(",")
                    .filter_map(|token| token.parse().ok())
                    .collect()
            })
            .collect();

        Line {
            start: Coordinate {
                x: split[0][0],
                y: split[0][1],
            },
            end: Coordinate {
                x: split[1][0],
                y: split[1][1],
            },
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

fn mark_affected_coordinates_in_map(map: &mut HashMap<Coordinate, u32>, line: &Line) {
    if line.is_horizontal() {
        let y = line.start.y;
        let start_x = if line.start.x < line.end.x {
            line.start.x
        } else {
            line.end.x
        };
        let end_x = if line.start.x > line.end.x {
            line.start.x
        } else {
            line.end.x
        };

        for x in start_x..end_x + 1 {
            let coordinate = Coordinate { x, y };
            let entry = map.entry(coordinate).or_insert(0);
            *entry += 1;
        }
    } else if line.is_vertical() {
        let x = line.start.x;
        let start_y = if line.start.y < line.end.y {
            line.start.y
        } else {
            line.end.y
        };
        let end_y = if line.start.y > line.end.y {
            line.start.y
        } else {
            line.end.y
        };

        for y in start_y..end_y + 1 {
            let coordinate = Coordinate { x, y };
            let entry = map.entry(coordinate).or_insert(0);
            *entry += 1;
        }
    } else {
        let mut x = line.start.x;
        let mut y = line.start.y;
        let distance = (line.end.x as i32 - line.start.x as i32).abs() as u32;

        let increment_x = line.end.x > line.start.x;
        let increment_y = line.end.y > line.start.y;
        for _ in 0..distance + 1 {
            let coordinate = Coordinate { x, y };
            let entry = map.entry(coordinate).or_insert(0);
            *entry += 1;

            if increment_x {
                x += 1;
            } else {
                x -= 1;
            }
            if increment_y {
                y += 1;
            } else {
                y -= 1;
            }
        }
    }
}

fn part_1(input: &[&str]) {
    let lines: Vec<Line> = input
        .iter()
        .map(|line| Line::new(line))
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();
    let mut map: HashMap<Coordinate, u32> = HashMap::new();

    for line in lines.iter() {
        mark_affected_coordinates_in_map(&mut map, line);
    }

    let dangerous_spot_count = map
        .values()
        .filter(|&&entry| entry > 1)
        .collect::<Vec<&u32>>()
        .len();
    println!("Part 1: {:?}", dangerous_spot_count);
}

fn part_2(input: &[&str]) {
    let lines: Vec<Line> = input.iter().map(|line| Line::new(line)).collect();
    let mut map: HashMap<Coordinate, u32> = HashMap::new();

    for line in lines.iter() {
        mark_affected_coordinates_in_map(&mut map, line);
    }

    let dangerous_spot_count = map
        .values()
        .filter(|&&entry| entry > 1)
        .collect::<Vec<&u32>>()
        .len();
    println!("Part 2: {:?}", dangerous_spot_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    part_1(&input);
    part_2(&input);
}
