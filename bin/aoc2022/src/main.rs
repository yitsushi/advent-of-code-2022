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
        _ => Box::new(aoc::MissingSolution::new()),
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
        assert!(!answer.is_err());
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
}
