use aoc::math::Vec2D;

mod grid;

/// Day 14: Regolith Reservoir
///
/// Using your scan, simulate the falling sand.
/// How many units of sand come to rest before sand starts flowing into the abyss below?
///
/// URL: <https://adventofcode.com/2022/day/14>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "498,4 -> 498,6 -> 496,6",
///   "503,4 -> 502,4 -> 502,9 -> 494,9",
/// ];
///
/// let mut solver = solution::day14::Solution::new();
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
///   "498,4 -> 498,6 -> 496,6",
///   "503,4 -> 502,4 -> 502,9 -> 494,9",
/// ];
///
/// let mut solver = solution::day14::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    cave: grid::Grid,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day14::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            let coordinates = line.split(" -> ")
                .map(|part| {
                    let group = part
                        .split(',')
                        .map(|v| v.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    match group[..] {
                        [x, y] => Vec2D::new(x, y),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<Vec2D<i64>>>();

            for i in 0..coordinates.len()-1 {
                let from = coordinates.get(i).unwrap();
                let to = coordinates.get(i+1).unwrap();

                self.cave.add_wall(from.clone(), to.clone());
            }
        }

        self.cave.add_source(Vec2D::new(500, 0));
    }

    fn part1(&mut self) -> String {
        let mut cycles = 0;
        while self.cave.cycle() {
            cycles += 1;
        }

        format!("{}", cycles)
    }

    fn part2(&mut self) -> String {
        let min = self.cave.min.clone();
        let max = self.cave.max.clone();
        self.cave.add_wall(
            Vec2D::new(min.x - 1000, max.y + 2),
            Vec2D::new(max.x + 1000, max.y + 2),
        );

        let mut cycles = 0;
        while self.cave.cycle() {
            cycles += 1;
        }

        format!("{}", cycles)
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
        assert_eq!(Solution::new().name(), "day14::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day14").unwrap());

        assert_eq!(solver.part1(), format!("{}", 24));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day14").unwrap());

        assert_eq!(solver.part2(), format!("{}", 93));
    }
}
