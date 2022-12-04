use std::str::FromStr;

use self::play::Play;

mod play;
mod hand;
mod outcome;

/// Day 2: Rock Paper Scissors
///
/// What would your total score be if everything goes exactly according to your
/// strategy guide?
///
/// URL: <https://adventofcode.com/2022/day/2>
///
/// # Example
///
/// ```
/// use aoc::Solver;
///
/// let mut solver = solution::day02::Solution::new();
/// solver.read_lines(vec![
///     "A X",
///     "B Y",
///     "C Z",
///     "A Z",
/// ].iter().map(|s| s.to_string()).collect());
///
/// let part1_solution = solver.part1();
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    plays: Vec<Play>
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day02::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        let plays: Vec<Play> = lines
            .iter()
            .map(|s| { Play::from_str(s).expect("input parse error") })
            .collect();

        self.plays = plays
    }

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
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day02::Solution")
    }

    #[test]
    fn example1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day02").unwrap());

        assert_eq!(solver.part1(), format!("{}", 15));
        assert_eq!(solver.part2(), format!("{}", 12));
    }
}
