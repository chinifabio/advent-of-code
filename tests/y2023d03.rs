use iter_tools::Itertools;

pub(crate) fn solve() -> (i32, i32) {
    let input = include_str!("../inputs/2023/03.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let dirs = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let parts = get_engine_parts(&input, &dirs);
    let solution_1 = parts.iter().map(|p| p.value).sum::<i32>();
    let solution_2 = get_gear_ratios(&parts).iter().sum::<i32>();

    (solution_1, solution_2)
}

struct EnginePart {
    value: i32,
    _symbol: char,
    symbol_pos: (i32, i32),
}

impl EnginePart {
    fn new(value: i32, symbol: char, symbol_pos: (i32, i32)) -> Self {
        Self {
            value,
            _symbol: symbol,
            symbol_pos,
        }
    }
}

fn get_engine_parts(input: &[Vec<char>], directions: &[(i32, i32)]) -> Vec<EnginePart> {
    let mut output = Vec::new();
    let mut value = 0;
    let mut has_symbol = false;
    let mut symbol_pos = (0, 0);
    let mut symbol = '.';
    let len = 140;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            match input[row][col].to_digit(10) {
                Some(digit) => {
                    value = value * 10 + digit as i32;
                    has_symbol |= !directions.iter().all(|(c, r)| {
                        let new_row = row as i32 + r;
                        let new_col = col as i32 + c;
                        if new_row < 0 || new_row >= len {
                            return true;
                        }
                        if new_col < 0 || new_col >= len {
                            return true;
                        }
                        let x = input[new_row as usize][new_col as usize];
                        if x == '.' || x.is_ascii_digit() {
                            return true;
                        }
                        symbol = x;
                        symbol_pos = (new_col, new_row);
                        false
                    });
                }
                None => {
                    if value != 0 && has_symbol {
                        output.push(EnginePart::new(value, symbol, symbol_pos));
                    }
                    value = 0;
                    has_symbol = false;
                    symbol = '.';
                    symbol_pos = (0, 0);
                }
            }
        }
    }
    output
}

fn get_gear_ratios(input: &[EnginePart]) -> Vec<i32> {
    let mut output = Vec::new();
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i].symbol_pos == input[j].symbol_pos {
                output.push(input[i].value * input[j].value)
            }
        }
    }
    output
}

#[test]
fn run() {
    assert_eq!(solve(), (525119, 76504829));
}
