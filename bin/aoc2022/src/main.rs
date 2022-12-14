use std::time::Instant;

use aoc::Solver;
use aoc::args::{Args, part::Part, day::Day};
use clap::Parser;
use aoc::io::{Filesystem, LocalFilesystem};

fn main() {
    let args = Args::parse();
    let fs = LocalFilesystem{};

    match run(&args, &fs) {
        Ok(value) => println!("{}", value),
        Err(err) => eprintln!("error: {}", err),
    }
}

fn run(args: &Args, fs: &impl Filesystem) -> Result<String, String> {
    let filename = match &args.input {
        Some(name) => name.clone(),
        None => input_filename(fs, &args.day, &args.part)
    };

    let input = match fs.read_file(&filename) {
        Ok(inp) => inp,
        Err(err) => { return Err(format!("{}", err)) },
    };

    let mut solver = get_solver(&args.day);

    let mut start_time = Instant::now();
    solver.read_lines(input);
    if args.time_it {
        eprintln!(" -- Bootstrap solver: {:?}", start_time.elapsed());
    }

    start_time = Instant::now();
    let answer = match args.part {
        Part::Part1 => solver.part1(),
        Part::Part2 => solver.part2(),
    };
    if args.time_it {
        eprintln!(" -- Solution: {:?}", start_time.elapsed());
    }

    Ok(answer)
}

fn input_filename(fs: &impl Filesystem, day: &Day, part: &Part) -> String {
    let file_path_without_part = format!("input/day{:0>2}", day);
    let file_path_with_part = format!("input/day{:0>2}-part{}", day, part);

    if fs.path_exists(&file_path_with_part) {
        file_path_with_part
    } else {
        file_path_without_part
    }
}

fn get_solver(day: &Day) -> Box<dyn Solver> {
    match day {
        Day::Day01 => Box::new(solution::day01::Solution::new()),
        Day::Day02 => Box::new(solution::day02::Solution::new()),
        Day::Day03 => Box::new(solution::day03::Solution::new()),
        Day::Day04 => Box::new(solution::day04::Solution::new()),
        Day::Day05 => Box::new(solution::day05::Solution::new()),
        Day::Day06 => Box::new(solution::day06::Solution::new()),
        Day::Day07 => Box::new(solution::day07::Solution::new()),
        Day::Day08 => Box::new(solution::day08::Solution::new()),
        Day::Day09 => Box::new(solution::day09::Solution::new()),
        Day::Day10 => Box::new(solution::day10::Solution::new()),
        Day::Day11 => Box::new(aoc::MissingSolution::new()),
        Day::Day12 => Box::new(aoc::MissingSolution::new()),
        Day::Day13 => Box::new(aoc::MissingSolution::new()),
        Day::Day14 => Box::new(aoc::MissingSolution::new()),
        Day::Day15 => Box::new(aoc::MissingSolution::new()),
        Day::Day16 => Box::new(aoc::MissingSolution::new()),
        Day::Day17 => Box::new(aoc::MissingSolution::new()),
        Day::Day18 => Box::new(aoc::MissingSolution::new()),
        Day::Day19 => Box::new(aoc::MissingSolution::new()),
        Day::Day20 => Box::new(aoc::MissingSolution::new()),
        Day::Day21 => Box::new(aoc::MissingSolution::new()),
        Day::Day22 => Box::new(aoc::MissingSolution::new()),
        Day::Day23 => Box::new(aoc::MissingSolution::new()),
        Day::Day24 => Box::new(aoc::MissingSolution::new()),
        Day::Day25 => Box::new(aoc::MissingSolution::new()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use aoc::args::{part::Part, day::Day, Args};
    use aoc::io::Error;
    use super::Filesystem;

    struct InMemoryFilesystem {
        valid_paths: HashMap<String, Vec<String>>,
    }

    impl InMemoryFilesystem {
        fn new() -> Self {
            let mut map = HashMap::new();
            map.insert(
                "input/day01".into(),
                vec![
                    "100", "200",
                    "",
                    "20", "400",
                    "",
                    "100",
                    "",
                    "150", "30", "20",
                    "",
                ].iter().map(|c| c.to_string() ).collect(),
            );
            map.insert("input/day02-part1".into(), Vec::new());
            map.insert("input/day02-part2".into(), Vec::new());

            InMemoryFilesystem{
                valid_paths: map,
            }
        }
    }

    impl Filesystem for InMemoryFilesystem {
        fn path_exists(&self, path: &str) -> bool {
            self.valid_paths.contains_key(path)
        }

        fn read_file(&self, path: &str) -> Result<Vec<String>, Error> {
            match self.valid_paths.get(path) {
                None => Err(Error::ReadError(path.to_string())),
                Some(content) => Ok(content.clone()),
            }
        }
    }

    impl InMemoryFilesystem {
        fn remove(&mut self, path: &str) {
            self.valid_paths.remove(path);
        }

        fn add(&mut self, path: String, content: Vec<String>) {
            self.valid_paths.insert(path, content);
        }
    }

    #[test]
    fn input_filename() {
        let fs = InMemoryFilesystem::new();

        // No part1 and part2 files
        assert_eq!(super::input_filename(&fs, &Day::Day01, &Part::Part1), "input/day01".to_string());
        assert_eq!(super::input_filename(&fs, &Day::Day01, &Part::Part2), "input/day01".to_string());
        // Has separate part1 and part2 files
        assert_eq!(super::input_filename(&fs, &Day::Day02, &Part::Part1), "input/day02-part1".to_string());
        assert_eq!(super::input_filename(&fs, &Day::Day02, &Part::Part2), "input/day02-part2".to_string());
    }

    #[test]
    fn run() {
        let args = Args{
            day: Day::Day01,
            part: Part::Part1,
            time_it: false,
            input: None,
        };
        let fs = InMemoryFilesystem::new();

        let answer = super::run(&args, &fs);
        assert!(!answer.is_err());
        assert_eq!(answer.unwrap(), "420".to_string());
    }

    #[test]
    fn run_no_input() {
        let args = Args{
            day: Day::Day01,
            part: Part::Part1,
            time_it: false,
            input: None,
        };
        let mut fs = InMemoryFilesystem::new();
        fs.remove("input/day01");

        let answer = super::run(&args, &fs);
        assert!(answer.is_err());
    }

    #[test]
    fn run_custom_input() {
        let args = Args{
            day: Day::Day02,
            part: Part::Part1,
            time_it: false,
            input: Some("input/random".into()),
        };
        let mut fs = InMemoryFilesystem::new();
        fs.add("input/random".into(), Vec::new());

        let answer = super::run(&args, &fs);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), "0".to_string());
    }

    #[test]
    fn run_custom_input_no_input() {
        let args = Args{
            day: Day::Day01,
            part: Part::Part1,
            time_it: false,
            input: Some("input/random".into()),
        };
        let fs = InMemoryFilesystem::new();

        let answer = super::run(&args, &fs);
        assert!(answer.is_err());
    }

    // To make sure we load the right solver.
    #[test]
    fn get_solver() {
        let days = vec![
            (Day::Day01, true), (Day::Day02, true), (Day::Day03, true), (Day::Day04, true), (Day::Day05, true),
            (Day::Day06, true), (Day::Day07, true), (Day::Day08, true), (Day::Day09, true), (Day::Day10, true),
            (Day::Day11, false), (Day::Day12, false), (Day::Day13, false), (Day::Day14, false), (Day::Day15, false),
            (Day::Day16, false), (Day::Day17, false), (Day::Day18, false), (Day::Day19, false), (Day::Day20, false),
            (Day::Day21, false), (Day::Day22, false), (Day::Day23, false), (Day::Day24, false), (Day::Day25, false),
        ];

        for (day, implemented) in days {
            if implemented {
                assert_eq!(super::get_solver(&day).name(), &format!("day{}::Solution", day))
            } else {
                assert_eq!(super::get_solver(&day).name(), "MissingSolution")
            }
        }
    }
}
