use std::str::FromStr;

use self::movement::Movement;

mod movement;
mod field;

/// Day 9: Rope Bridge
///
/// Simulate your complete hypothetical series of motions. How many positions
/// does the tail of the rope visit at least once?
///
/// URL: <https://adventofcode.com/2022/day/9>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "R 4",
///   "U 4",
///   "L 3",
///   "D 1",
///   "R 4",
///   "D 1",
///   "L 5",
///   "R 2",
/// ];
///
/// let mut solver = solution::day09::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part1_solution = solver.part1();
/// ```
///
/// # Part 2
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "R 4",
///   "U 4",
///   "L 3",
///   "D 1",
///   "R 4",
///   "D 1",
///   "L 5",
///   "R 2",
/// ];
///
/// let mut solver = solution::day09::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    motions: Vec<Movement>
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day09::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        self.motions = lines.iter()
            .map(|line| match Movement::from_str(line) {
                Ok(m) => m,
                Err(err) => panic!("{}", err)
            })
            .collect();
    }

    fn part1(&mut self) -> String {
        let rope = self.motions.iter()
            .copied()
            .flat_map(|m| m.split())
            .fold(field::Rope::new(1), |r, m| r.map(m));

        format!("{}", rope.tail_history.len())
    }

    fn part2(&mut self) -> String {
        let rope = self.motions.iter()
            .copied()
            .flat_map(|m| m.split())
            .fold(field::Rope::new(9), |r, m| r.map(m));

        format!("{}", rope.tail_history.len())
    }
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self { Self::default() }
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day09::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day09").unwrap());

        assert_eq!(solver.part1(), format!("{}", 13));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day09-p2").unwrap());

        assert_eq!(solver.part2(), format!("{}", 36));
    }
}
