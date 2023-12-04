use std::str::FromStr;

use crate::utils::conversions::Parser;

pub(crate) fn solve() -> (i32, i32) {
    let input = include_str!("../../inputs/2023/02.txt").to_vec::<Game>("\n");

    let solution_1 = input.iter()
        .filter(|x| x.is_valid(&12, &13, &14))
        .map(|x| x.id)
        .sum::<i32>();

    let solution_2 = input.iter()
        .map(|x| x.power())
        .sum::<i32>();

    return (solution_1, solution_2);
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<(i32, i32, i32)>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (game, sets) = input.split_once(": ").unwrap();

        let id = game.split_once(" ").unwrap().1.parse::<i32>().unwrap();

        let mut temp = Vec::new();
        for set in sets.split("; ") {
            let (mut r, mut g, mut b) = (0, 0, 0);
            for pair in set.split(", ") {
                let (num, color) = pair.split_once(" ").unwrap();
                match color {
                    "red" => r = num.parse::<i32>().unwrap(),
                    "green" => g = num.parse::<i32>().unwrap(),
                    "blue" => b = num.parse::<i32>().unwrap(),
                    _ => panic!("Invalid key"),
                }
            }
            temp.push((r, g, b));
        }
        Ok(Game { id, sets: temp })
    }
}

impl Game {
    fn is_valid(&self, max_red: &i32, max_green: &i32, max_blue: &i32) -> bool {
        self.sets.iter().all(|(r, g, b)| r <= max_red && g <= max_green && b <= max_blue)
    }

    fn power(&self) -> i32 {
        let r = self.sets.iter().map(|s| s.0).max().unwrap();
        let g = self.sets.iter().map(|s| s.1).max().unwrap();
        let b = self.sets.iter().map(|s| s.2).max().unwrap();
        r * g * b
    }
}

#[test]
fn test() {
    assert_eq!(solve(), (2149, 71274));
}