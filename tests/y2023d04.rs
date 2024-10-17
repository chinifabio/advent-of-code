use std::str::FromStr;

use advent_of_code::utils::conversions::Parser;

pub(crate) fn solve() -> (u32, u32) {
    let mut input = include_str!("../inputs/2023/04.txt").to_vec::<Card>("\n");

    let solution_1 = input.iter().map(|c| c.get_points()).sum::<u32>();

    for i in 0..input.len() {
        let n = input[i].quantity;
        for j in 0..input[i].get_n_matches() as usize {
            input[i + 1 + j].increase_quantity(n);
        }
    }
    let solution_2 = input.iter().map(|c| c.quantity).sum::<u32>();

    (solution_1, solution_2)
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
    _id: i32,
    quantity: u32,
}

impl Card {
    fn new(id: i32, winning_numbers: Vec<i32>, my_numbers: Vec<i32>) -> Self {
        Card {
            _id: id,
            winning_numbers,
            my_numbers,
            quantity: 1,
        }
    }

    fn get_points(&self) -> u32 {
        let mut points = -1;
        for number in self.my_numbers.iter() {
            if self.winning_numbers.contains(number) {
                points += 1;
            }
        }
        if points == -1 {
            return 0;
        }
        2u32.pow(points as u32)
    }

    fn get_n_matches(&self) -> u32 {
        self.my_numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count() as u32
    }

    fn increase_quantity(&mut self, n: u32) {
        self.quantity += n;
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = input.split_once(": ").unwrap();

        let id: i32 = card.split(" ").last().unwrap().parse().unwrap();

        let mut winning_numbers = Vec::new();
        let mut my_numbers = Vec::new();

        let (raw_winning_numbers, raw_my_numbers) = numbers.split_once(" | ").unwrap();

        for number in raw_winning_numbers.split(" ") {
            if let Ok(n) = number.parse::<i32>() {
                winning_numbers.push(n)
            }
        }

        for number in raw_my_numbers.split(" ") {
            if let Ok(n) = number.parse::<i32>() {
                my_numbers.push(n)
            }
        }

        Ok(Card::new(id, winning_numbers, my_numbers))
    }
}

#[test]
fn run() {
    assert_eq!(solve(), (20107, 8172507));
}
