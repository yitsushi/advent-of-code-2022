use std::str::FromStr;

use itertools::Itertools;

mod cave;

const TIMEOUT: i32 = 30;

/// Day 16: Proboscidea Volcanium
///
/// Work out the steps to release the most pressure in 30 minutes. What is the most pressure you
/// can release?
///
/// URL: <https://adventofcode.com/2022/day/16>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
///   "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
///   "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
///   "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
///   "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
///   "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
///   "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
///   "Valve HH has flow rate=22; tunnel leads to valve GG",
///   "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
///   "Valve JJ has flow rate=21; tunnel leads to valve II",
/// ];
///
/// let mut solver = solution::day16::Solution::new();
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
///   "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
///   "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
///   "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
///   "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
///   "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
///   "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
///   "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
///   "Valve HH has flow rate=22; tunnel leads to valve GG",
///   "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
///   "Valve JJ has flow rate=21; tunnel leads to valve II",
/// ];
///
/// let mut solver = solution::day16::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    cave: cave::Cave,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day16::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        lines.iter()
            .enumerate()
            .map(|(idx, line)| {
                let mut valve = cave::Valve::from_str(line).unwrap();
                valve.set_bit(idx);

                valve
            })
            .for_each(|v| self.cave.add_valve(v));
    }

    fn part1(&mut self) -> String {
        let mut cave = self.cave.clone();
        cave.prepare();
        let max = cave.walk(TIMEOUT).values()
            .copied()
            .max()
            .unwrap();

        format!("{}", max)
    }

    fn part2(&mut self) -> String {
        let mut cave = self.cave.clone();
        cave.prepare();
        let max = cave.walk(TIMEOUT - 4).iter()
            .tuple_combinations()
            .filter(|(myself, elephant)| myself.0 & elephant.0 == 0)
            .map(|(myself, elephant)| myself.1 + elephant.1)
            .max()
            .unwrap();

        format!("{}", max)
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
        assert_eq!(Solution::new().name(), "day16::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day16").unwrap());

        assert_eq!(solver.part1(), format!("{}", 1651));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day16").unwrap());

        assert_eq!(solver.part2(), format!("{}", 1707));
    }
}
