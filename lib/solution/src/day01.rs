pub struct Solution {
    elfs: Vec<Elf>,
}

impl aoc::Solution for Solution {
    fn part1(&self) -> String {
        format!("{}", self.elfs[0].sum)
    }

    fn part2(&self) -> String {
        let sum: i64 = self.elfs[..3].iter().map(|e| e.sum).sum();

        format!("{}", sum)
    }
}

impl Solution {
    pub fn from_lines(lines: Vec<String>) -> Box<dyn aoc::Solution> {
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

        Box::new(Solution{ elfs })
    }
}

struct Elf {
    bars: Vec<i64>,
    sum: i64,
}

impl Elf {
    fn new() -> Elf {
        Elf { bars: Vec::new(), sum: 0, }
    }

    fn add(&mut self, value: i64) {
        self.bars.push(value)
    }

    fn finalize(&mut self) {
        self.sum = self.bars.iter().sum();
    }
}

#[cfg(test)]
mod tests {
    use super::Elf;
    use super::Solution;

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

    fn input(filename: &str) -> Vec<String> {
        match aoc::io::read_input(filename) {
            Ok(inp) => inp,
            Err(err) => { panic!("Error: {}", err); },
        }
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
        let solver = Solution::from_lines(lines());

        assert_eq!(solver.part1(), format!("{}", 200));
    }

    #[test]
    fn solution_part2() {
        let solver = Solution::from_lines(lines());

        assert_eq!(solver.part2(), format!("{}", 200 + 110 + 90));
    }

    #[test]
    fn example1() {
        let solver = Solution::from_lines(input("tests/fixtures/day01"));

        assert_eq!(solver.part1(), format!("{}", 24000));
        assert_eq!(solver.part2(), format!("{}", 45000));
    }
}
