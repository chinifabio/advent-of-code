use std::collections::HashSet;

use advent_of_code::utils::{grid::*, point::*};
use iter_tools::Itertools;

pub(crate) fn solve() -> Result<(u32, u32), ()> {
    let raw = include_str!("../inputs/2024/06.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let map = Grid::from_matrix(raw);

    let solution_1 = count_unique_positions(map.clone());
    let solution_2 = 0;

    Ok((solution_1 as u32, solution_2 as u32))
}

fn find_start(map: &Grid<char>) -> Point {
    for i in 0..map.height {
        for j in 0..map.width {
            let p = Point::new(j as i32, i as i32);
            if map[p] == '^' {
                return p;
            }
        }
    }
    panic!("Start char not found :(")
}

fn count_unique_positions(map: Grid<char>) -> usize {
    let mut positions: HashSet<Point> = HashSet::new();
    let mut direction = UP;
    let mut pos = find_start(&map);
    let mut next_pos = pos + direction;
    while map.contains(&next_pos) {
        if map[next_pos] == '#' {
            direction = direction.clockwise();
            next_pos = pos + direction;
        }
        positions.insert(pos);
        pos = next_pos;
        next_pos = pos + direction;
    }
    positions.insert(pos);
    positions.len()
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (5086, 0));
    Ok(())
}
