use std::collections::HashMap;
use std::env;

#[derive(Clone)]
struct Number {
    row: usize,
    column: usize,
}

#[derive(Clone)]
struct Board {
    numbers: HashMap<u32, Number>,
    rows: Vec<usize>,
    columns: Vec<usize>,
    row_count: usize,
    column_count: usize,
    has_won: bool,
}

impl Board {
    fn new(numbers: HashMap<u32, Number>, row_count: usize, column_count: usize) -> Board {
        Board {
            numbers,
            rows: vec![0; row_count],
            columns: vec![0; column_count],
            row_count,
            column_count,
            has_won: false,
        }
    }

    fn mark_number(&mut self, number: &u32) -> bool {
        let is_winning_number = if let Some(x) = self.numbers.get(number) {
            self.rows[x.row] += 1;
            self.columns[x.column] += 1;

            if self.rows[x.row] >= self.row_count || self.columns[x.column] >= self.column_count {
                self.has_won = true;
                true
            } else {
                false
            }
        } else {
            false
        };

        self.numbers.remove(number);
        is_winning_number
    }

    fn sum_unmarked_numbers(&self) -> u32 {
        self.numbers.keys().sum()
    }
}

fn split_input(string: &String) -> (Vec<u32>, Vec<Board>) {
    let intermediate_vector = string
        .split("\n\n")
        .filter(|token| !token.is_empty())
        .map(|token| token.trim())
        .collect::<Vec<&str>>();

    let draw_order = intermediate_vector
        .iter()
        .cloned()
        .take(1)
        .collect::<Vec<&str>>()[0]
        .split(",")
        .filter_map(|token| token.parse().ok())
        .collect();

    let mut boards = Vec::new();
    for board in intermediate_vector.iter().cloned().skip(1) {
        let rows: Vec<&str> = board.split("\n").collect();
        let mut column_count: usize = 0;

        let mut numbers_map: HashMap<u32, Number> = HashMap::new();
        for (row_index, row) in rows.iter().enumerate() {
            let numbers: Vec<u32> = row
                .split(" ")
                .filter_map(|number| number.parse().ok())
                .collect();

            if column_count == 0 {
                column_count = numbers.len();
            }

            for (column_index, &number) in numbers.iter().enumerate() {
                numbers_map.insert(
                    number,
                    Number {
                        row: row_index,
                        column: column_index,
                    },
                );
            }
        }
        boards.push(Board::new(numbers_map, rows.len(), column_count));
    }

    (draw_order, boards)
}

fn part_1(draw_order: &Vec<u32>, boards: &[Board]) {
    let mut winning_number: u32 = 0;
    let mut sum_of_unmarked_numbers: u32 = 0;
    let mut boards = boards.to_vec();
    'outer: for drawn_number in draw_order.iter() {
        for board in boards.iter_mut() {
            if board.mark_number(drawn_number) {
                winning_number = *drawn_number;
                sum_of_unmarked_numbers = board.sum_unmarked_numbers();
                break 'outer;
            }
        }
    }

    println!("Part 1: {:?}", winning_number * &sum_of_unmarked_numbers);
}

fn part_2(draw_order: &[u32], boards: &[Board]) {
    let mut winning_number: u32 = 0;
    let mut sum_of_unmarked_numbers: u32 = 0;
    let mut boards = boards.to_vec();

    for drawn_number in draw_order.iter() {
        if boards.len() == 1 && boards[0].mark_number(drawn_number) {
            sum_of_unmarked_numbers = boards[0].sum_unmarked_numbers();
            winning_number = *drawn_number;
            break;
        }

        for board in boards.iter_mut() {
            board.mark_number(drawn_number);
        }

        boards.retain(|board| !board.has_won);
    }

    println!("Part 2: {:?}", winning_number * &sum_of_unmarked_numbers);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = utils::read_file(path).expect("Error reading file.");
    let (draw_order, boards) = split_input(&input);
    part_1(&draw_order, &boards);
    part_2(&draw_order, &boards);
}
