use std::collections::{HashMap, HashSet};

use advent_of_code::utils::{grid::*, point::*};
use iter_tools::Itertools;

pub(crate) fn solve() -> Result<(u32, u32), ()> {
    let raw = include_str!("../inputs/2024/06.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let map = Grid::from_matrix(raw);

    let positions = count_unique_positions(map.clone());
    let solution_1 = positions.len();
    // si potrebbe evitare la clone della map perchè basta resettare il carattere modificato
    // una volta checkato il loop
    let solution_2 = positions
        .into_iter()
        .map(|p| {
            let mut map_clone = map.clone();
            if map_clone[p] != '^' {
                map_clone[p] = '#';
            }
            map_clone
        })
        .filter(|m| contains_loop(m))
        .count();

    Ok((solution_1 as u32, solution_2 as u32))
}

fn find_start(map: &Grid<char>) -> Option<Vec2> {
    map.iter_positions().find(|&p| map[p] == '^')
}

fn count_unique_positions(map: Grid<char>) -> HashSet<Vec2> {
    let mut positions: HashSet<Vec2> = HashSet::new();

    let mut direction = UP;
    let mut pos = find_start(&map).expect("No start found :(");
    let mut next_pos = pos + direction;

    while map.contains(&next_pos) {
        if map[next_pos] == '#' {
            direction = direction.clockwise();
            next_pos = pos + direction;
            continue;
        }

        positions.insert(pos);

        pos = next_pos;
        next_pos = pos + direction;
    }

    positions.insert(pos);
    positions
}

fn contains_loop(map: &Grid<char>) -> bool {
    let mut breadcrumbs: HashMap<Vec2, Vec2> = HashMap::new();

    let mut direction = UP;
    let mut pos = find_start(&map).expect("No start found :(");
    let mut next_pos = pos + direction;

    while map.contains(&next_pos) {
        if map[next_pos] == '#' {
            direction = direction.clockwise();
            next_pos = pos + direction;
            continue;
        }

        match breadcrumbs.get(&pos) {
            Some(crumb_dir) => {
                if *crumb_dir == direction {
                    return true;
                }
            }
            None => {
                breadcrumbs.insert(pos, direction);
            }
        }

        pos = next_pos;
        next_pos = pos + direction;
    }

    false
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (5086, 1770));
    Ok(())
}
