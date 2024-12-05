use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn solve() -> Result<(u32, u32), ()> {
    let wordsearch = include_str!("../inputs/2024/04.txt")
        .parse::<WordSearch>()
        .unwrap();

    let solution_1 = wordsearch.search_word("XMAS").len();

    let centers = wordsearch
        .search_word("MAS")
        .into_iter()
        .filter(|(_, _, (a, b))| (b - a).abs() != 1)
        .map(|(i, j, (di, dj))| (i as i32 + di, j as i32 + dj));

    let mut center_counts = HashMap::new();
    for center in centers {
        *center_counts.entry(center).or_insert(0) += 1;
    }

    let solution_2 = center_counts.values().filter(|&&count| count > 1).count();

    Ok((solution_1 as u32, solution_2 as u32))
}

struct WordSearch {
    haystack: Vec<Vec<char>>,
}

type Word = (usize, usize, (i32, i32));

impl WordSearch {
    fn new(haystack: Vec<Vec<char>>) -> Self {
        assert!(haystack.len() > 1);
        assert!(haystack[0].len() > 1);
        WordSearch { haystack }
    }

    fn search_word(&self, word: &str) -> Vec<Word> {
        let mut words = Vec::new();

        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for i in 0..self.haystack.len() {
            for j in 0..self.haystack[0].len() {
                if self.haystack[i][j] == word.chars().nth(0).unwrap() {
                    for dir in self.recursion(word, 1, i, j, directions.clone()) {
                        words.push((i, j, dir));
                    }
                }
            }
        }

        words
    }

    // TODO: use an array of bit to flag if a direction is valid instead of hammering the memory with allocs
    fn recursion(
        &self,
        word: &str,
        idx: usize,
        i: usize,
        j: usize,
        directions: Vec<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        if idx >= word.len() || directions.is_empty() {
            return directions;
        }

        let target_char = word.chars().nth(idx).unwrap();
        let mut new_directions = Vec::new();

        for (di, dj) in directions.into_iter() {
            let ni = i as i32 + di * idx as i32;
            let nj = j as i32 + dj * idx as i32;

            if ni >= 0
                && ni < self.haystack.len() as i32
                && nj >= 0
                && nj < self.haystack[0].len() as i32
            {
                if self.haystack[ni as usize][nj as usize] == target_char {
                    new_directions.push((di, dj));
                }
            }
        }

        return self.recursion(word, idx + 1, i, j, new_directions);
    }
}

impl FromStr for WordSearch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WordSearch::new(
            s.lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect(),
        ))
    }
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (2462, 1877));
    Ok(())
}
