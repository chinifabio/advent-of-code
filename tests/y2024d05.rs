use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use iter_tools::Itertools;

pub(crate) fn solve() -> Result<(u32, u32), ()> {
    let day5 = include_str!("../inputs/2024/05.txt")
        .parse::<Day5>()
        .unwrap();

    let solution_1 = day5
        .updates
        .iter()
        .filter(|u| u.is_correct(&day5.update_rules))
        .map(|u| u.mean_value)
        .sum();

    let mut successors: HashMap<u32, Vec<u32>> = HashMap::new();
    for (a, b) in &day5.update_rules {
        successors.entry(*a).or_default().push(*b);
    }
    let solution_2 = day5
        .updates
        .into_iter()
        .filter(|u| !u.is_correct(&day5.update_rules))
        .map(|u| {
            u.into_correct(&day5.update_rules.iter().copied().collect())
                .mean_value
        })
        .sum();

    Ok((solution_1, solution_2))
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
    mean_value: u32,
    indexes: HashMap<u32, usize>,
}

struct Day5 {
    update_rules: Vec<(u32, u32)>,
    updates: Vec<Update>,
}

impl Update {
    pub fn new(pages: Vec<u32>) -> Self {
        let mut indexes = HashMap::new();
        for (i, e) in pages.iter().enumerate() {
            indexes.insert(*e, i);
        }
        let mean_value = pages[pages.len() / 2];
        Update {
            pages,
            indexes,
            mean_value,
        }
    }

    pub fn is_correct(&self, update_rules: &Vec<(u32, u32)>) -> bool {
        update_rules
            .iter()
            .all(|(a, b)| match (self.indexes.get(a), self.indexes.get(b)) {
                (Some(&a_idx), Some(&b_idx)) => a_idx < b_idx,
                _ => true,
            })
    }

    pub fn into_correct(self, update_rules: &HashSet<(u32, u32)>) -> Self {
        let Self { mut pages, .. } = self;
        pages.sort_by(|a, b| {
            if update_rules.contains(&(*a, *b)) {
                Ordering::Less
            } else if update_rules.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        Self::new(pages)
    }
}

impl FromStr for Day5 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("\n\n");
        let update_rules = split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                l.split("|")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();
        let updates = split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .map(|v| Update::new(v))
            .collect_vec();
        Ok(Day5 {
            update_rules,
            updates,
        })
    }
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (5639, 5273));
    Ok(())
}
