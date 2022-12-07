use std::str::FromStr;

mod command;
mod fs;

/// Day 7: No Space Left On Device
///
/// Find all of the directories with a total size of at most 100000.
/// What is the sum of the total sizes of those directories?
///
/// URL: <https://adventofcode.com/2022/day/7>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "$ cd /",
///   "$ ls",
///   "dir a",
///   "14848514 b.txt",
///   "8504156 c.dat",
///   "dir d",
///   "$ cd a",
///   "$ ls",
///   "dir e",
///   "29116 f",
///   "2557 g",
///   "62596 h.lst",
///   "$ cd e",
///   "$ ls",
///   "584 i",
///   "$ cd ..",
///   "$ cd ..",
///   "$ cd d",
///   "$ ls",
///   "4060174 j",
///   "8033020 d.log",
///   "5626152 d.ext",
///   "7214296 k",
/// ];
///
/// let mut solver = solution::day07::Solution::new();
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
///   "$ cd /",
///   "$ ls",
///   "dir a",
///   "14848514 b.txt",
///   "8504156 c.dat",
///   "dir d",
///   "$ cd a",
///   "$ ls",
///   "dir e",
///   "29116 f",
///   "2557 g",
///   "62596 h.lst",
///   "$ cd e",
///   "$ ls",
///   "584 i",
///   "$ cd ..",
///   "$ cd ..",
///   "$ cd d",
///   "$ ls",
///   "4060174 j",
///   "8033020 d.log",
///   "5626152 d.ext",
///   "7214296 k",
/// ];
///
/// let mut solver = solution::day07::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    fs: fs::Directory,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day07::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        let mut root = fs::Directory::new("/".into());

        let mut path: Vec<String> = Vec::new();

        for line in lines {
            if line.starts_with("$ ") {
                match command::Command::from_str(line.strip_prefix("$ ").unwrap_or("")) {
                    Ok(command::Command::List) => continue,
                    Ok(command::Command::ChDir(dir)) => {
                        if dir == "/" {
                            path = Vec::new();
                        } else if dir == ".." {
                            path.pop();
                        } else {
                            path.push(dir);
                        }
                    },
                    Err(err) => { eprintln!("error: {}", err); return }
                };
            } else {
                match fs::Entry::from_str(&line) {
                    Ok(fs::Entry::Directory(name)) => {
                        root = root.add_dir(path.clone(), fs::Directory::new(name))
                    },
                    Ok(fs::Entry::File(name, size)) => {
                        root = root.add_file(path.clone(), name, size)
                    },
                    Err(err) => { eprintln!("error: {}", err); return }
                }
            }
        }

        self.fs = root
    }

    fn part1(&mut self) -> String {
        let c: u64 = self.fs.all_dirs()
            .iter()
            .filter(|d| d.size <= 100000)
            .map(|d| d.size)
            .sum();

        format!("{}", c)
    }

    fn part2(&mut self) -> String {
        const FULL: u64 = 70000000;
        const TARGET: u64 = 30000000;

        let missing_free_space = TARGET - (FULL - self.fs.size);

        let mut candidates: Vec<u64> = self.fs.all_dirs()
            .iter()
            .filter(|d| d.size >= missing_free_space)
            .map(|d| d.size)
            .collect::<Vec<u64>>();

        candidates.sort();

        format!("{}", candidates.first().unwrap())
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
        assert_eq!(Solution::new().name(), "day07::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day07").unwrap());

        assert_eq!(solver.part1(), format!("{}", 95437));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day07").unwrap());

        assert_eq!(solver.part2(), format!("{}", 24933642));
    }
}
