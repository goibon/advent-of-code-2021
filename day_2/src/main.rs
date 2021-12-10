use std::env;

struct Command<'a> {
    direction: &'a str,
    amount: u32,
}

fn split_input(string: &String) -> Vec<Command> {
    string
        .split("\n")
        .filter(|token| !token.is_empty())
        .map(|token| {
            let split: Vec<&str> = token.split(' ').collect();
            Command {
                direction: split[0],
                amount: split[1].parse().unwrap(),
            }
        })
        .collect()
}

const FORWARD_DIRECTION: &str = "forward";
const DOWNWARD_DIRECTION: &str = "down";
const UPWARD_DIRECTION: &str = "up";

fn follow_commands_part_1(commands: &Vec<Command>) -> (u32, u32) {
    let mut horizontal_position: u32 = 0;
    let mut vertical_position: u32 = 0;
    for command in commands {
        match command.direction {
            FORWARD_DIRECTION => {
                horizontal_position += command.amount;
            }
            DOWNWARD_DIRECTION => {
                vertical_position += command.amount;
            }
            UPWARD_DIRECTION => {
                vertical_position -= command.amount;
            }
            &_ => break,
        }
    }

    (horizontal_position, vertical_position)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    let (horizontal_position, vertical_position) = follow_commands_part_1(&input);
    let part_1 = horizontal_position * vertical_position;
    println!("Part 1: {:?}", part_1);
}
