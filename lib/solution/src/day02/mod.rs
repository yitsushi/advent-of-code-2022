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
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn example1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day02").unwrap());

        assert_eq!(solver.part1(), format!("{}", 15));
        assert_eq!(solver.part2(), format!("{}", 12));
    }
}
