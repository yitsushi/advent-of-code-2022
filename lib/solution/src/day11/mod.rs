use std::collections::HashMap;

use monkey::Monkey;

mod monkey;
mod token;

/// Day 11: Monkey in the Middle
///
/// Figure out which monkeys to chase by counting how many items they inspect over 20 rounds. What
/// is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
///
/// URL: <https://adventofcode.com/2022/day/11>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "Monkey 0:",
///   "  Starting items: 79, 98",
///   "  Operation: new = old * 19",
///   "  Test: divisible by 23",
///   "    If true: throw to monkey 2",
///   "    If false: throw to monkey 3",
///   "",
///   "Monkey 1:",
///   "  Starting items: 54, 65, 75, 74",
///   "  Operation: new = old + 6",
///   "  Test: divisible by 19",
///   "    If true: throw to monkey 2",
///   "    If false: throw to monkey 0",
///   "",
///   "Monkey 2:",
///   "  Starting items: 79, 60, 97",
///   "  Operation: new = old * old",
///   "  Test: divisible by 13",
///   "    If true: throw to monkey 1",
///   "    If false: throw to monkey 3",
///   "",
///   "Monkey 3:",
///   "  Starting items: 74",
///   "  Operation: new = old + 3",
///   "  Test: divisible by 17",
///   "    If true: throw to monkey 0",
///   "    If false: throw to monkey 1",
/// ];
///
/// let mut solver = solution::day11::Solution::new();
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
///   "Monkey 0:",
///   "  Starting items: 79, 98",
///   "  Operation: new = old * 19",
///   "  Test: divisible by 23",
///   "    If true: throw to monkey 2",
///   "    If false: throw to monkey 3",
///   "",
///   "Monkey 1:",
///   "  Starting items: 54, 65, 75, 74",
///   "  Operation: new = old + 6",
///   "  Test: divisible by 19",
///   "    If true: throw to monkey 2",
///   "    If false: throw to monkey 0",
///   "",
///   "Monkey 2:",
///   "  Starting items: 79, 60, 97",
///   "  Operation: new = old * old",
///   "  Test: divisible by 13",
///   "    If true: throw to monkey 1",
///   "    If false: throw to monkey 3",
///   "",
///   "Monkey 3:",
///   "  Starting items: 74",
///   "  Operation: new = old + 3",
///   "  Test: divisible by 17",
///   "    If true: throw to monkey 0",
///   "    If false: throw to monkey 1",
/// ];
///
/// let mut solver = solution::day11::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    monkeys: Vec<Monkey>,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day11::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
       self.monkeys = lines.into_iter()
            .filter(|f| !f.is_empty())
            .collect::<Vec<String>>()
            .chunks(6)
            .map(|block| {
                block.iter().fold(Monkey::default(), |mut monkey, line| {
                    monkey.parse_line(line.to_string());
                    monkey
                })
            })
            .collect()
            ;
    }

    fn part1(&mut self) -> String {
        let mut monkeys = self.monkeys.clone();
        let mut inspects: HashMap<i64, i64> = HashMap::new();
        let modi = monkeys.iter().map(|m| m.modi()).product();

        for _ in 0..20 {
            (0..monkeys.len()).for_each(|idx| {
                let current = monkeys[idx].clone();
                let (targets, monkey) = current.process(modi, true);

                add_ispection(&mut inspects, idx as i64, targets.len() as i64);

                monkeys[idx] = monkey;

                for (target, value) in targets {
                    let idx = target as usize;
                    monkeys[idx] = monkeys[idx].clone().push(value);
                }
            });
        };

        let (first, second) = top_two(&inspects);

        format!("{}", first * second)
    }

    fn part2(&mut self) -> String {
        let mut monkeys = self.monkeys.clone();
        let mut inspects: HashMap<i64, i64> = HashMap::new();
        let modi = monkeys.iter().map(|m| m.modi()).product();

        for _ in 0..10000 {
            (0..monkeys.len()).for_each(|idx| {
                let current = monkeys[idx].clone();
                let (targets, monkey) = current.process(modi, false);

                add_ispection(&mut inspects, idx as i64, targets.len() as i64);

                monkeys[idx] = monkey;

                for (target, value) in targets {
                    let idx = target as usize;
                    monkeys[idx] = monkeys[idx].clone().push(value);
                }
            });
        };

        let (first, second) = top_two(&inspects);

        format!("{}", first * second)
    }
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self { Self::default() }
}

fn add_ispection(inspects: &mut HashMap<i64, i64>, idx: i64, value: i64) {
    let add = if let Some(v) = inspects.get(&idx) {
        v + value
    } else {
        value
    };

    inspects.insert(idx, add);
}

fn top_two(inspects: &HashMap<i64, i64>) -> (i64, i64) {
    let mut values = inspects.values().cloned().collect::<Vec<i64>>();
    values.sort();

    (values.pop().unwrap(), values.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day11::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day11").unwrap());

        assert_eq!(solver.part1(), format!("{}", 10605));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day11").unwrap());

        assert_eq!(solver.part2(), format!("{}", 2713310158_i64));
    }
}
