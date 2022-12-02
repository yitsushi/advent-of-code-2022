use std::time::Instant;

use aoc::Solver;
use aoc::args::{Args, part::Part, day::Day};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let filename = input_filename(&args.day, &args.part);

    let input = match aoc::io::read_input(&filename) {
        Ok(inp) => inp,
        Err(err) => { println!("Error: {}", err); return },
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

    println!("{}", answer);
}

fn input_filename(day: &Day, part: &Part) -> String {
    let file_path_without_part = format!("input/day{:0>2}", day);
    let file_path_with_part = format!("input/day{:0>2}-part{}", day, part);

    if std::path::Path::new(&file_path_with_part).exists() {
        file_path_with_part
    } else {
        file_path_without_part
    }
}

fn get_solver(day: &Day) -> Box<dyn Solver> {
    match day {
        Day::Day01 => Box::new(solution::day01::Solution::new()),
        Day::Day02 => Box::new(solution::day02::Solution::new()),
        Day::Day03 => Box::new(aoc::MissingSolution::new()),
        Day::Day04 => Box::new(aoc::MissingSolution::new()),
        Day::Day05 => Box::new(aoc::MissingSolution::new()),
        Day::Day06 => Box::new(aoc::MissingSolution::new()),
        Day::Day07 => Box::new(aoc::MissingSolution::new()),
        Day::Day08 => Box::new(aoc::MissingSolution::new()),
        Day::Day09 => Box::new(aoc::MissingSolution::new()),
        Day::Day10 => Box::new(aoc::MissingSolution::new()),
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
    use aoc::args::{part::Part, day::Day};

    #[test]
    fn input_filename() {
        assert_eq!(super::input_filename(&Day::Day01, &Part::Part1), "input/day01".to_string());
    }
}
