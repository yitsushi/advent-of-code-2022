pub struct Solution {
    elfs: Vec<Elf>,
}

impl aoc::Solution for Solution {
    fn part1(&self) {
        println!("Part1: {}", self.elfs[0].sum)
    }

    fn part2(&self) {
        let sum: i64 = self.elfs[..3].iter().map(|e| e.sum).sum();
        println!("Part1: {}", sum)
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
