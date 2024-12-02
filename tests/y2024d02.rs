use std::str::FromStr;

use advent_of_code::utils::conversions::Parser;

pub(crate) fn solve() -> Result<(u32, u32), ()> {
    let reports = include_str!("../inputs/2024/02.txt").to_vec::<Report>("\n");

    let solution_1 = reports.iter().filter(|r| r.safe).count() as u32;
    let solution_2 = reports.iter().filter(|r| r.weakly_safe).count() as u32;

    Ok((solution_1, solution_2))
}

#[derive(Debug)]
struct Report {
    _levels: Vec<i32>,
    safe: bool,
    weakly_safe: bool,
}

impl Report {
    fn is_safe(levels: &Vec<i32>) -> bool {
        let is_increasing = levels.windows(2).all(|w| w[0] < w[1]);
        let is_decreasing = levels.windows(2).all(|w| w[0] > w[1]);

        // if the other conditions hold then maybe i can check only the first and the last values (using the len of the levels)
        // the <= is already granted by the < / > (without =) in the previous checks
        let is_adjacent_diff_valid = levels
            .windows(2)
            .all(|w| (w[0] - w[1]).abs() >= 1 && (w[0] - w[1]).abs() <= 3);

        (is_increasing || is_decreasing) && is_adjacent_diff_valid
    }

    fn new(levels: Vec<i32>) -> Self {
        let is_safe = Self::is_safe(&levels);

        let is_weakly_safe = (0..levels.len()).any(|i| {
            let mut temp = levels.clone();
            temp.remove(i);
            Self::is_safe(&temp)
        });

        Self {
            _levels: levels,
            safe: is_safe,
            weakly_safe: is_weakly_safe,
        }
    }
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        if levels.len() == 0 {
            Err(())
        } else {
            Ok(Self::new(levels))
        }
    }
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (220, 296));
    Ok(())
}
