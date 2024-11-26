pub(crate) fn solve() -> Result<(u64, u64), ()> {
    let mut _input = include_str!("../inputs/2023/06.txt");

    let races = [
        Race::new(61, 643),
        Race::new(70, 1184),
        Race::new(90, 1362),
        Race::new(66, 1041),
    ];

    let solution_1 = races.iter().map(|r| r.win_situations()).product::<u64>();
    let solution_2 = Race::new(61709066, 643118413621041).win_situations();

    Ok((solution_1, solution_2))
}

struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn new(time: u64, record_distance: u64) -> Self {
        Self {
            time,
            record_distance,
        }
    }

    fn run(&self, button_time: u64) -> u64 {
        (self.time - button_time) * button_time
    }

    fn win_situations(&self) -> u64 {
        let mut win_situations = 0;
        for button_time in 0..self.time {
            if self.run(button_time) > self.record_distance {
                win_situations += 1;
            }
        }
        win_situations
    }
}

#[test]
fn run() -> Result<(), ()> {
    assert_eq!(solve()?, (293046, 35150181));
    Ok(())
}
