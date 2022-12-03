use std::collections::HashSet;

#[derive(Default)]
pub struct Solution {
    rucksacks: Vec<Rucksack>,
}

impl aoc::Solver for Solution {
    fn read_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            let compartment_length = line.len() / 2;

            let sack = Rucksack{
                compartment1: line[..compartment_length].as_bytes().to_vec(),
                compartment2: line[compartment_length..].as_bytes().to_vec(),
            };

            self.rucksacks.push(sack);
        }
    }

    fn part1(&self) -> String {
        let result: i32 = self.rucksacks
            .iter()
            .map(|sack| sack.misplaced_items())
            .map(|items| items.iter().map(|s| u8_to_priority(*s)).sum::<i32>())
            .sum()
        ;

        format!("{}", result)
    }

    fn part2(&self) -> String {
        let mut chunks: Vec<Vec<Rucksack>> = Vec::new();

        for i in 0..(self.rucksacks.len()/3) {
            chunks.push(self.rucksacks[(i*3)..(i*3)+3].to_vec());
        }

        let results: i32 = chunks
            .iter()
            .filter_map(|block| {
                common_item(&block.iter().map(|s| s.full()).collect())
            })
            .map(u8_to_priority)
            .sum()
            ;

        format!("{}", results)
    }
}

impl Solution {
    pub fn new() -> Self {
        Solution{ rucksacks: Vec::new() }
    }
}

#[derive(Default, Clone)]
struct Rucksack {
    compartment1: Vec<u8>,
    compartment2: Vec<u8>,
}

impl Rucksack {
    fn misplaced_items(&self) -> Vec<u8> {
        let set1 = self.compartment1.clone().into_iter().collect::<HashSet<u8>>();
        let set2 = self.compartment2.clone().into_iter().collect::<HashSet<u8>>();

        set1.intersection(&set2)
            .copied()
            .collect::<Vec<u8>>()
    }

    fn full(&self) -> Vec<u8> {
        self.compartment1.clone().into_iter().chain(self.compartment2.clone().into_iter()).collect()
    }
}

fn common_item(list: &Vec<Vec<u8>>) -> Option<u8> {
    let mut base = list.first().unwrap().clone().into_iter().collect::<HashSet<u8>>();

    for item in list {
        let current = item.clone().into_iter().collect::<HashSet<u8>>();
        base = base.intersection(&current).copied().collect::<HashSet<u8>>();
    };

    base.into_iter().collect::<Vec<u8>>().first().copied()
}

fn u8_to_priority(ch: u8) -> i32 {
    if ch > 96 {
        (ch - 96) as i32
    } else {
        (ch - 38) as i32
    }
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn example1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day03").unwrap());

        assert_eq!(solver.part1(), format!("{}", 157));
        assert_eq!(solver.part2(), format!("{}", 70));
    }

    #[test]
    fn common_item() {
        let cases: Vec<(Vec<&str>, Option<u8>)> = vec![
            (vec!["asdfm", "mqwer", "zxmcv"], Some('m' as u8)),
            (vec!["asdfm", "qwer", "zxmcv"], None),
        ];

        for (input, expected) in cases {
            let list = input.iter().map(|s| s.as_bytes().to_vec()).collect();

            assert_eq!(super::common_item(&list), expected);
        }
    }

    #[test]
    fn u8_to_priority() {
        let cases: Vec<(char, i32)> = vec![
            ('a', 1),
            ('z', 26),
            ('A', 27),
            ('Z', 52),
        ];

        for (input, expected) in cases {
            assert_eq!(super::u8_to_priority(input as u8), expected);
        }
    }
}
