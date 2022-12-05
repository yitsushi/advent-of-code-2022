use self::stack::{Stack, Crate};
use self::instruction::Instruction;
use std::str::FromStr;

mod stack;
mod instruction;

/// Day 5: Supply Stacks
///
/// After the rearrangement procedure completes, what crate ends up on top of
/// each stack?
///
/// URL: <https://adventofcode.com/2022/day/5>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "    [D]    ",
///   "[N] [C]    ",
///   "[Z] [M] [P]",
///   " 1   2   3 ",
///   "",
///   "move 1 from 2 to 1",
///   "move 3 from 1 to 3",
///   "move 2 from 2 to 1",
///   "move 1 from 1 to 2",
/// ];
///
/// let mut solver = solution::day05::Solution::new();
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
///   "    [D]    ",
///   "[N] [C]    ",
///   "[Z] [M] [P]",
///   " 1   2   3 ",
///   "",
///   "move 1 from 2 to 1",
///   "move 3 from 1 to 3",
///   "move 2 from 2 to 1",
///   "move 1 from 1 to 2",
/// ];
///
/// let mut solver = solution::day05::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day05::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            if line.is_empty() {
                continue
            }

            if line.contains('[') {
                // Parse crates
                self.place_crates(line);
            } else if line.starts_with("move") {
                // Parse operations
                let value = Instruction::from_str(&line).expect("parse error");
                self.instructions.push(value);
            }
        }

        for stack in self.stacks.iter_mut() {
            stack.reverse()
        }
    }

    fn part1(&mut self) -> String {
        for ins in self.instructions.iter() {
            for _ in 0..ins.count() {
                match self.stacks[ins.from()-1].pop() {
                    Some(value) => self.stacks[ins.to()-1].push(value),
                    None => return format!("can't do that: {:?}", ins),
                };
            }
        }

        self.top_crates()
    }

    fn part2(&mut self) -> String {
        for ins in self.instructions.iter() {
            let mut inter: Vec<Option<Crate>> = Vec::new();
            for _ in 0..ins.count() {
                inter.push(self.stacks[ins.from()-1].pop());
            }
            inter.reverse();
            for c in inter {
                match c {
                    Some(value) => self.stacks[ins.to()-1].push(value),
                    None => return format!("can't do that: {:?}", ins),
                };
            }
        }

        self.top_crates()
    }
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self { Self::default() }

    fn place_crates(&mut self, line: String) {
        for (idx, chunk) in line.as_bytes().chunks(4).enumerate() {

            if self.stacks.len() <= idx {
                self.stacks.push(Stack::default());
            }

            match Crate::from_str(&String::from_utf8_lossy(chunk)) {
                Ok(item) => self.stacks[idx].push(item),
                Err(_) => continue,
            }
        }
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|s| s.top())
            .copied()
            .map(|c| c.name())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day05::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day05").unwrap());

        assert_eq!(solver.part1(), "CMZ".to_string());
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day05").unwrap());

        assert_eq!(solver.part2(), "MCD".to_string());
    }
}
