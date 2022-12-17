mod packet;

use self::packet::Packet;

/// Day 13: Distress Signal
///
/// Determine which pairs of packets are already in the right order.
/// What is the sum of the indices of those pairs?
///
/// URL: <https://adventofcode.com/2022/day/13>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "[1,1,3,1,1]",
///   "[1,1,5,1,1]",
///   "",
///   "[[1],[2,3,4]]",
///   "[[1],4]",
/// ];
///
/// let mut solver = solution::day13::Solution::new();
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
///   "[1,1,3,1,1]",
///   "[1,1,5,1,1]",
///   "",
///   "[[1],[2,3,4]]",
///   "[[1],4]",
/// ];
///
/// let mut solver = solution::day13::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    packet_pairs: Vec<PacketPair>,
}

type PacketPair = (Packet, Packet);

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day13::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        self.packet_pairs = lines.into_iter()
            .filter(|f| !f.is_empty())
            .collect::<Vec<String>>()
            .chunks(2)
            .map(|block| match block {
                [fst, snd] =>
                    (Packet::from_string(fst.clone()), Packet::from_string(snd.clone())),
                _ => unreachable!(),
            })
            .collect::<Vec<PacketPair>>();
    }

    fn part1(&mut self) -> String {
        let sum = self.packet_pairs.iter()
            .enumerate()
            .filter_map(|(idx, (fst, snd))|
                if fst <= snd { Some(idx+1) } else { None }
            )
            .sum::<usize>();

        format!("{}", sum)
    }

    fn part2(&mut self) -> String {
        let extras: Vec<Packet> = vec![
            Packet::from_string("[[2]]".to_string()),
            Packet::from_string("[[6]]".to_string()),
        ];

        let mut full = self.packet_pairs.iter()
            .flat_map(|(fst, snd)| vec![fst, snd])
            .chain(extras.iter())
            .cloned()
            .collect::<Vec<Packet>>();

        full.sort();

        let res: usize = full.iter()
            .enumerate()
            .filter_map(|(idx, p)|
                if extras.contains(p) {
                    Some(idx + 1)
                } else {
                    None
                }
            )
            .product();

        format!("{}", res)
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
        assert_eq!(Solution::new().name(), "day13::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day13").unwrap());

        assert_eq!(solver.part1(), format!("{}", 13));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day13").unwrap());

        assert_eq!(solver.part2(), format!("{}", 140));
    }

    #[test]
    fn input_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("../../input/day13").unwrap());

        assert_eq!(solver.part2(), format!("{}", 25935));
    }
}
