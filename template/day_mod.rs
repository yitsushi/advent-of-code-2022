/// Day ${day}: NAME
///
/// DESCRIPTION
///
/// URL: <https://adventofcode.com/2022/day/${day}>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
/// ];
///
/// let mut solver = solution::day${padded_day}::Solution::new();
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
/// ];
///
/// let mut solver = solution::day${padded_day}::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution { }

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day${padded_day}::Solution"
    }

    fn read_lines(&mut self, _: Vec<String>) {
        todo!()
    }

    fn part1(&self) -> String {
        todo!()
    }

    fn part2(&self) -> String {
        todo!()
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
        assert_eq!(Solution::new().name(), "day${padded_day}::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day${padded_day}").unwrap());

        assert_eq!(solver.part1(), format!("{}", 0));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day${padded_day}").unwrap());

        assert_eq!(solver.part2(), format!("{}", 0));
    }
}
