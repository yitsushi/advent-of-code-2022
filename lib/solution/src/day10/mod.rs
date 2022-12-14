use std::str::FromStr;

use self::{instruction::Instruction, computer::{Program, Memory}};

mod computer;
mod instruction;

/// Day 10: Cathode-Ray Tube
///
/// Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is
/// the sum of these six signal strengths?
///
/// URL: <https://adventofcode.com/2022/day/10>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "noop",
///   "addx 3",
///   "addx -5",
/// ];
///
/// let mut solver = solution::day10::Solution::new();
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
///   "noop",
///   "addx 3",
///   "addx -5",
/// ];
///
/// let mut solver = solution::day10::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    instructions: Vec<Instruction>,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day10::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        self.instructions = lines.iter()
            .filter_map(|s| if let Ok(v) = Instruction::from_str(s) {
                Some(v)
            } else {
                None
            })
            .collect()
            ;
    }

    fn part1(&mut self) -> String {
        let mut program = Program::new(Memory::new(1), self.instructions.clone());

        let (signals, _) = program.run();

        format!("{}", signals.iter().sum::<i32>())
    }

    fn part2(&mut self) -> String {
        let mut program = Program::new(Memory::new(1), self.instructions.clone());

        let (_, _) = program.run();

        // println!("{}", program);

        format!("{}", program).replace('#', "█")
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
        assert_eq!(Solution::new().name(), "day10::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day10").unwrap());

        assert_eq!(solver.part1(), format!("{}", 13140));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day10").unwrap());

        assert_eq!(solver.part2(), format!("██..██..██..██..██..██..██..██..██..██..\n███...███...███...███...███...███...███.\n████....████....████....████....████....\n█████.....█████.....█████.....█████.....\n██████......██████......██████......████\n███████.......███████.......███████.....\n"));
    }
}
