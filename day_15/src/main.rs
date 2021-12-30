use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{collections::HashMap, env};

fn split_input(string: &str) -> Vec<Vec<u32>> {
    string
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|row| {
            row.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn get_adjacent_indices(
    column: usize,
    row: usize,
    max_column: usize,
    max_row: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_indices: Vec<(usize, usize)> = Vec::new();

    if column > 0 {
        adjacent_indices.push((column - 1, row))
    };
    if column < max_column {
        adjacent_indices.push((column + 1, row))
    };
    if row > 0 {
        adjacent_indices.push((column, row - 1))
    };
    if row < max_row {
        adjacent_indices.push((column, row + 1))
    };

    adjacent_indices
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstras(input: &[Vec<u32>]) -> Option<usize> {
    let target_y = input.len() - 1;
    let target_x = input[target_y].len() - 1;

    let mut nodes_to_explore = BinaryHeap::new();
    nodes_to_explore.push(State {
        cost: 0,
        position: (0, 0),
    });
    let mut f_score: HashMap<(usize, usize), u32> = HashMap::from([((0, 0), 0)]);

    while let Some(State { cost, position }) = nodes_to_explore.pop() {
        if position == (target_y, target_x) {
            return Some(cost);
        }

        if cost > *f_score.entry(position).or_insert(u32::MAX) as usize {
            continue;
        }

        for neighbor in get_adjacent_indices(position.0, position.1, target_x, target_y) {
            let next = State {
                cost: cost + input[neighbor.1][neighbor.0] as usize,
                position: neighbor,
            };

            let f_score_entry = f_score.entry(next.position).or_insert(u32::MAX);
            if next.cost < *f_score_entry as usize {
                nodes_to_explore.push(next);
                // Relaxation, we have now found a better way
                *f_score_entry = next.cost as u32;
            }
        }
    }

    None
}

fn part_1(input: &[Vec<u32>]) {
    println!("Part 1: {}", dijkstras(input).unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = std::fs::read_to_string(path).expect("Error reading file.");
    let input = split_input(&input);

    part_1(&input);
}
