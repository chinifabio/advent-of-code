use regex::Regex;

pub(crate) fn solve() -> Result<(u32, u32), ()> {
    let mut _input = include_str!("../inputs/2024/03.txt");

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut muls = Vec::new();
    let mut enable = true;
    for cap in mul_regex.captures_iter(_input) {
        match &cap[0] {
            "do()" => enable = true,
            "don't()" => enable = false,
            _ => muls.push(Mul {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                enabled: enable,
            }),
        }
    }

    let solution_1 = muls.iter().map(|mul| mul.x * mul.y).sum::<u32>();
    let solution_2 = muls.iter().filter(|mul| mul.enabled).map(|mul| mul.x * mul.y).sum::<u32>();

    Ok((solution_1, solution_2))
}

struct Mul {
    x: u32,
    y: u32,
    enabled: bool,
}



#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (163931492, 76911921));
    Ok(())
}
