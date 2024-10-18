use std::{collections::HashSet, str::FromStr};

use regex::Regex;

fn solve() -> Result<(u64, u64), ()> {
    let input = include_str!("../inputs/2023/05.txt").parse::<Almanac>()?;

    let solution_1 = search_min(input.seeds.clone(), |s| input.map_seed(s));

    let solution_2 = search_min_2(input);

    Ok((solution_1, solution_2))
}

fn search_min<F>(seeds: Vec<u64>, mapper: F) -> u64
where
    F: Fn(u64) -> u64,
{
    seeds.iter().map(|&s| mapper(s)).min().unwrap()
}

fn search_min_2(input: Almanac) -> u64 {
    let reversed_categories = input.reverse_categories();
    let mut points = HashSet::new();

    for i in 0..reversed_categories.len() {
        let cat = &reversed_categories[i];
        cat.maps.iter().for_each(|m| {
            points.insert(m.dest);
            points.insert(m.dest + m.length - 1);
        });

        if i < reversed_categories.len() - 1 {
            let mut new_points = HashSet::new();
            points.iter().for_each(|p| {
                let new_p = reversed_categories[i + 1]
                    .maps
                    .iter()
                    .find(|m| m.contains(*p))
                    .map(|m| m.map_item(*p))
                    .unwrap_or(*p);
                new_points.insert(new_p);
            });
            points.extend(new_points);
        }
    }

    let mut test_points = input
        .seed_ranges
        .iter()
        .flat_map(|r| vec![r.start, r.end])
        .collect::<HashSet<_>>();
    test_points.extend(points.iter().filter(|p| {
        input
            .seed_ranges
            .iter()
            .any(|r| r.start < **p && **p < r.end)
    }));

    let mapper = |s| input.map_seed(s);
    search_min(test_points.into_iter().collect(), mapper)
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<SeedRange>,
    categories: Vec<Category>,
}

#[derive(Debug, Clone)]
struct SeedRange {
    start: u64,
    end: u64,
    _length: u64,
}

#[derive(Debug, Clone)]
struct Category {
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
struct Map {
    dest: u64,
    source: u64,
    length: u64,
}

impl Almanac {
    fn map_seed(&self, seed: u64) -> u64 {
        self.categories.iter().fold(seed, |acc, c| {
            c.maps
                .iter()
                .find(|m| m.contains(acc))
                .map(|m| m.map_item(acc))
                .unwrap_or(acc)
        })
    }

    fn reverse_categories(&self) -> Vec<Category> {
        self.categories
            .iter()
            .cloned()
            .rev()
            .map(|c| Category {
                maps: c
                    .maps
                    .into_iter()
                    .map(|m| Map::new(m.source, m.dest, m.length))
                    .collect(),
            })
            .collect()
    }
}

impl Map {
    fn new(dest: u64, source: u64, length: u64) -> Self {
        assert!(length > 0);
        Map {
            dest,
            source,
            length,
        }
    }

    fn map_item(&self, item: u64) -> u64 {
        assert!(self.contains(item));
        let offset = item - self.source;
        self.dest + offset
    }

    fn contains(&self, item: u64) -> bool {
        item >= self.source && item < self.source + self.length
    }
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces: Vec<&str> = s.split("\n\n").collect();
        let seeds: Vec<u64> = pieces
            .drain(..1)
            .map(|s| {
                let re = Regex::new(r"\d+").unwrap();
                re.find_iter(s)
                    .map(|m| m.as_str().parse().unwrap())
                    .collect()
            })
            .next()
            .unwrap();
        let mut seeds_iter = seeds.clone().into_iter();
        let mut seed_ranges = Vec::new();
        for _ in 0..seeds.len() / 2 {
            let start = seeds_iter.next().unwrap();
            let length = seeds_iter.next().unwrap();
            seed_ranges.push(SeedRange {
                start,
                end: start + length - 1,
                _length: length,
            });
        }
        let categories = pieces
            .iter()
            .map(|&p| {
                let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
                let maps = re
                    .captures_iter(p)
                    .map(|c| {
                        Map::new(
                            c[1].parse().unwrap(),
                            c[2].parse().unwrap(),
                            c[3].parse().unwrap(),
                        )
                    })
                    .collect();
                Category { maps }
            })
            .collect();
        Ok(Almanac {
            seeds,
            seed_ranges,
            categories,
        })
    }
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (313045984, 20283860));
    Ok(())
}
