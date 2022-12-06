use std::collections::HashSet;

/// Day 6: Tuning Trouble
///
/// How many characters need to be processed before the first start-of-packet
/// marker is detected?
///
/// URL: <https://adventofcode.com/2022/day/6>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz"];
///
/// let mut solver = solution::day06::Solution::new();
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
/// let lines: Vec<&str> = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz"];
///
/// let mut solver = solution::day06::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    signal: String,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day06::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        if let Some(signal) = lines.first() {
            self.signal = signal.to_string();
        } else {
            panic!("invalid input")
        };
    }

    fn part1(&mut self) -> String {
        if self.signal.len() < 4 {
            return "[no solution]".to_string()
        }

        for idx in 0..(self.signal.len()-3) {
            let current = &self.signal[idx..idx+4].chars().collect::<HashSet<char>>();
            if current.len() == 4 {
                return format!("{}", idx+4);
            }
        }

        "[no solution]".to_string()
    }

    fn part2(&mut self) -> String {
        if self.signal.len() < 14 {
            return "[no solution]".to_string()
        }

        for idx in 0..(self.signal.len()-13) {
            let current = &self.signal[idx..idx+14].chars().collect::<HashSet<char>>();
            if current.len() == 14 {
                return format!("{}", idx+14);
            }
        }

        "[no solution]".to_string()
    }
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self { Self::default() }
}

#[cfg(test)]
mod tests {
    use aoc::Solver;

    use super::Solution;

    fn solver_part1(input: &str, expected: &str) {
        let mut solver = Solution::new();
        solver.read_lines(vec![input.to_string()]);
        assert_eq!(solver.part1(), expected.to_string())
    }

    fn solver_part2(input: &str, expected: &str) {
        let mut solver = Solution::new();
        solver.read_lines(vec![input.to_string()]);
        assert_eq!(solver.part2(), expected.to_string())
    }

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day06::Solution")
    }

    #[test]
    #[should_panic]
    fn empty_input() {
        let mut solver = Solution::new();
        solver.read_lines(vec![]);
    }

    #[test]
    fn part1_too_short() { solver_part1("a", "[no solution]"); }

    #[test]
    fn part2_too_short() { solver_part2("a", "[no solution]"); }

    #[test]
    fn part1_no_solution() { solver_part1("aaaaaaaaaaaaaaaaaaaa", "[no solution]"); }

    #[test]
    fn part2_no_solution() { solver_part2("aaaaaaaaaaaaaaaaaaaa", "[no solution]"); }

    #[test]
    fn example1_part1() { solver_part1("bvwbjplbgvbhsrlpgdmjqwftvncz", "5"); }

    #[test]
    fn example2_part1() { solver_part1("nppdvjthqldpwncqszvftbrmjlhg", "6"); }

    #[test]
    fn example3_part1() { solver_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10"); }

    #[test]
    fn example4_part1() { solver_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11"); }

    #[test]
    fn example1_part2() { solver_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19"); }

    #[test]
    fn example2_part2() { solver_part2("bvwbjplbgvbhsrlpgdmjqwftvncz", "23"); }

    #[test]
    fn example3_part2() { solver_part2("nppdvjthqldpwncqszvftbrmjlhg", "23"); }

    #[test]
    fn example4_part2() { solver_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29"); }

    #[test]
    fn example5_part2() { solver_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26"); }
}
