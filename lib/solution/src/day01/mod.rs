/// Day 1: Calorie Counting
///
/// Find the Elf carrying the most Calories. How many total Calories is that
/// Elf carrying?
///
/// URL: <https://adventofcode.com/2022/day/1>
///
/// # Example
///
/// ```
/// use aoc::Solver;
///
/// let mut solver = solution::day01::Solution::new();
/// solver.read_lines(vec![
///     "10", "20",
///     "",
///     "40", "20",
///     "",
///     "10", "15",
///     "",
/// ].iter().map(|s| s.to_string()).collect());
///
/// let part1_solution = solver.part1();
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    elves: Vec<Elf>,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day01::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        let mut elfs: Vec<Elf> = Vec::new();

        let mut elf: Elf = Elf::new();
        for line in lines {
            if line.is_empty() {
                elf.finalize();
                elfs.push(elf);
                elf = Elf::new();
                continue
            }

            elf.add(line.parse().unwrap());
        }

        elfs.sort_by(|a, b| b.sum.cmp(&a.sum));

        self.elves = elfs
    }

    fn part1(&mut self) -> String {
        format!("{}", self.elves[0].sum)
    }

    fn part2(&mut self) -> String {
        let sum: i64 = self.elves[..3].iter().map(|e| e.sum).sum();

        format!("{}", sum)
    }
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Elf with calories
///
/// # Example
///
/// ```
/// let mut elf = solution::day01::Elf::new();
///
/// elf.add(20);
/// elf.add(50);
/// elf.finalize();
///
/// assert_eq!(elf.sum, 70);
/// ```
#[derive(Default)]
pub struct Elf {
    bars: Vec<i64>,
    pub sum: i64,
}

impl Elf {
    pub fn new() -> Elf {
        Elf { bars: Vec::new(), sum: 0, }
    }

    /// Add calorie value to the Elf.
    pub fn add(&mut self, value: i64) {
        self.bars.push(value)
    }

    /// Finalize `sum`. It can be called multiple times, but without
    /// calling `finalize`, the value of `sum` will not be updated.
    pub fn finalize(&mut self) {
        self.sum = self.bars.iter().sum();
    }
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::{Elf, Solution};

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day01::Solution")
    }

    fn lines() -> Vec<String> {
        vec![
            "10", "20", "60", // 90
            "",
            "40", "30", // 70
            "",
            "200", // 200
            "",
            "20", "20", "30", "40",  // 110
            "",
        ].iter().map(|c| c.to_string() ).collect()
    }

    #[test]
    fn elf_finalizer() {
        let mut elf = Elf::new();
        elf.add(15);
        elf.add(20);
        elf.add(10);

        elf.finalize();

        assert_eq!(elf.sum, 45);
    }

    #[test]
    fn solution_part1() {
        let mut solver = Solution::new();
        solver.read_lines(lines());

        assert_eq!(solver.part1(), format!("{}", 200));
    }

    #[test]
    fn solution_part2() {
        let mut solver = Solution::new();
        solver.read_lines(lines());

        assert_eq!(solver.part2(), format!("{}", 200 + 110 + 90));
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day01").unwrap());

        assert_eq!(solver.part1(), format!("{}", 24000));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day01").unwrap());

        assert_eq!(solver.part2(), format!("{}", 45000));
    }
}
