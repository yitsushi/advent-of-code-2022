#[derive(Default)]
pub struct Solution { }

impl aoc::Solver for Solution {
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
    pub fn new() -> Self {
        Solution{}
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
        solver.read_lines(fs.read_file("tests/fixtures/empty").unwrap());

        assert_eq!(solver.part1(), format!("{}", 0));
        assert_eq!(solver.part2(), format!("{}", 0));
    }
}
