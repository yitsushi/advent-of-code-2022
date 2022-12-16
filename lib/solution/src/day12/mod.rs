use aoc::math::Vec2D;

use self::grid::Grid;

//mod map;
mod grid;
mod state;

/// Day 12: Hill Climbing Algorithm
///
/// What is the fewest steps required to move from your current position to the location that
/// should get the best signal?
///
/// URL: <https://adventofcode.com/2022/day/12>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "Sabqponm",
///   "abcryxxl",
///   "accszExk",
///   "acctuvwj",
///   "abdefghi",
/// ];
///
/// let mut solver = solution::day12::Solution::new();
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
///   "Sabqponm",
///   "abcryxxl",
///   "accszExk",
///   "acctuvwj",
///   "abdefghi",
/// ];
///
/// let mut solver = solution::day12::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    map: Grid,
    start: Vec2D<i64>,
    end: Vec2D<i64>,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day12::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        for (y, line) in lines.into_iter().enumerate() {
            for (x, ch) in line.into_bytes().iter().enumerate() {
                let pos = Vec2D::new(x as i64, y as i64);
                let value = match *ch as char {
                    'S' => {
                        self.start = pos.clone();
                        0
                    },
                    'E' => {
                        self.end = pos.clone();
                        b'z' - 0x61
                    },
                    v => v as u8 - 0x61,
                };
                self.map.add(Vec2D::new(x as i64, y as i64), value);
            }
        }
    }

    fn part1(&mut self) -> String {
        let result = self.map.walk(
            0,
            self.start.clone(),
            |_| self.end.clone(),
            |c| c == self.end,
            |from, to| from + 1 >= to,
        );
        match result {
            None => "solution not found".to_string(),
            Some(v) => format!("{}", v),
        }
    }

    fn part2(&mut self) -> String {
        let lowest = self.map.lowest_points();
        let result = self.map.walk(
            b'z'-0x61,
            self.end.clone(),
            |c| c.closest(&lowest),
            |c| lowest.contains(&c),
            |from, to| to + 1 >= from,
        );
        match result {
            None => "solution not found".to_string(),
            Some(v) => format!("{}", v),
        }
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
        assert_eq!(Solution::new().name(), "day12::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day12").unwrap());

        assert_eq!(solver.part1(), format!("{}", 31));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day12").unwrap());

        assert_eq!(solver.part2(), format!("{}", 29));
    }

    #[test]
    fn impossible_map() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day12-impossible").unwrap());

        assert_eq!(solver.part1(), format!("solution not found"));
    }
}
