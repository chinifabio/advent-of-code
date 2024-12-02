use iter_tools::Itertools;

pub(crate) fn solve() -> Result<(u32, u32), ()> {
    let input = include_str!("../inputs/2024/01.txt");

    let numbers = input
        .lines()
        .flat_map(|x| {
            x.split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
        })
        .collect::<Vec<u32>>();

    let left = numbers
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, x)| x)
        .sorted()
        .collect::<Vec<&u32>>();
    let right = numbers
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, x)| x)
        .sorted()
        .collect::<Vec<&u32>>();

    let solution_1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(**r))
        .sum::<u32>();

    let mut occurrences = std::collections::HashMap::new();
    for &number in &right {
        *occurrences.entry(number).or_insert(0) += 1;
    }

    let solution_2 = left
        .iter()
        .map(|x| *x * occurrences.get(x).unwrap_or(&0))
        .sum();

    Ok((solution_1, solution_2))
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (2264607, 19457120));
    Ok(())
}
