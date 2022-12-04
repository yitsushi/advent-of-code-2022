use std::str::FromStr;
use aoc::range_pair::RangePair;

/// Day 4: Camp Cleanup
///
/// In how many assignment pairs do the ranges overlap?
///
/// URL: <https://adventofcode.com/2022/day/4>
///
/// # Example
///
/// ```
/// use aoc::Solver;
///
/// let mut solver = solution::day04::Solution::new();
/// solver.read_lines(vec![
///     "2-4,6-8",
///     "2-3,4-5",
///     "5-7,7-9",
///     "2-8,3-7",
///     "6-6,4-6",
///     "2-6,4-8",
/// ].iter().map(|s| s.to_string()).collect());
///
/// let part1_solution = solver.part1();
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    groups: Vec<RangePair<i32>>,
}

impl aoc::Solver for Solution {
    fn read_lines(&mut self, lines: Vec<String>) {
        let pairs: Vec<RangePair<i32>> = lines.iter()
            .map(|s| {
                RangePair::from_str(s).expect("parse error")
            })
            .collect();

        self.groups = pairs;
    }

    fn part1(&self) -> String {
        let useless_pairs: usize = self.groups.iter().flat_map(|s| s.useless()).count();

        format!("{}", useless_pairs)
    }

    fn part2(&self) -> String {
        let overlap_pairs: usize = self.groups.iter().filter(|s| s.overlap()).count();

        format!("{}", overlap_pairs)
    }
}

impl Solution {
    pub fn new() -> Self {
        Solution{ groups: Vec::new() }
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
        solver.read_lines(fs.read_file("tests/fixtures/day04").unwrap());

        assert_eq!(solver.part1(), format!("{}", 2));
        assert_eq!(solver.part2(), format!("{}", 4));
    }
}
