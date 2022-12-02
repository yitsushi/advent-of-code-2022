use std::str::FromStr;

use self::play::Play;

mod play;
mod hand;
mod outcome;

#[derive(Default)]
pub struct Solution {
    plays: Vec<Play>
}

impl aoc::Solver for Solution {
    fn part1(&self) -> String {
        let score: i32 = self.plays
            .iter()
            .map(|play| play.score_part1() )
            .sum();

        score.to_string()
    }

    fn part2(&self) -> String {
        let score: i32 = self.plays
            .iter()
            .map(|play| play.score() )
            .sum();

        score.to_string()
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        let plays: Vec<Play> = lines
            .iter()
            .map(|s| { Play::from_str(s).expect("input parse error") })
            .collect();

        self.plays = plays
    }
}

impl Solution {
    pub fn new() -> Self {
        Solution{ plays: Vec::new() }
    }
}


#[cfg(test)]
mod tests {
    use aoc::Solver;

    use super::Solution;

    fn input(filename: &str) -> Vec<String> {
        match aoc::io::read_input(filename) {
            Ok(inp) => inp,
            Err(err) => { panic!("Error: {}", err); },
        }
    }

    #[test]
    fn example1() {
        let mut solver = Solution::new();
        solver.read_lines(input("tests/fixtures/day02"));

        assert_eq!(solver.part1(), format!("{}", 15));
        assert_eq!(solver.part2(), format!("{}", 12));
    }
}
