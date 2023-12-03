fn solve() -> (i32, i32) {
    let input = include_str!("../../inputs/2023/01.txt");

    let literals = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let solution_1 = input.lines().map(get_calibration_value).sum::<i32>();
    let solution_2 = input.lines().map(|line| get_real_calibration_value(line, &literals)).sum::<i32>();

    return (solution_1, solution_2);
}

fn get_calibration_value(input: &str) -> i32 {
    let nums: Vec<char> = input.chars().filter(|c| c.is_digit(10)).collect();

    format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
        .parse::<i32>()
        .unwrap()
}

fn get_real_calibration_value(input: &str, literals: &Vec<&str>) -> i32 {
    let mut temp = input.to_owned();

    for (i, name) in literals.iter().enumerate() {
        temp = temp.replace(name, format!("{}{}{}", name, i, name).as_str());
    }

    get_calibration_value(temp.as_str())
}

#[test]
fn test() {
    assert_eq!(solve(), (55017, 53539));
}