use std::time::Instant;

use aoc::Solution;
use aoc::args::{Args, part::Part, day::Day};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let file_path_without_part = format!("input/day{:0>2}", args.day);
    let file_path_with_part = format!("input/day{:0>2}-part{}", args.day, args.part);

    let filename = if std::path::Path::new(&file_path_with_part).exists() {
        file_path_with_part
    } else {
        file_path_without_part
    };

    let input = match aoc::io::read_input(&filename) {
        Ok(inp) => inp,
        Err(err) => { println!("Error: {}", err); return },
    };

    let mut start_time = Instant::now();

    let solver: Box<dyn Solution> = match args.day {
        Day::Day01 => solution::day01::Solution::from_lines(input),
        Day::Day02 => solution::day02::Solution::from_lines(input),
        Day::Day03 => solution::day03::Solution::from_lines(input),
        Day::Day04 => solution::day04::Solution::from_lines(input),
        Day::Day05 => solution::day05::Solution::from_lines(input),
        Day::Day06 => solution::day06::Solution::from_lines(input),
        Day::Day07 => solution::day07::Solution::from_lines(input),
        Day::Day08 => solution::day08::Solution::from_lines(input),
        Day::Day09 => solution::day09::Solution::from_lines(input),
        Day::Day10 => solution::day10::Solution::from_lines(input),
        Day::Day11 => solution::day11::Solution::from_lines(input),
        Day::Day12 => solution::day12::Solution::from_lines(input),
        Day::Day13 => solution::day13::Solution::from_lines(input),
        Day::Day14 => solution::day14::Solution::from_lines(input),
        Day::Day15 => solution::day15::Solution::from_lines(input),
        Day::Day16 => solution::day16::Solution::from_lines(input),
        Day::Day17 => solution::day17::Solution::from_lines(input),
        Day::Day18 => solution::day18::Solution::from_lines(input),
        Day::Day19 => solution::day19::Solution::from_lines(input),
        Day::Day20 => solution::day20::Solution::from_lines(input),
        Day::Day21 => solution::day21::Solution::from_lines(input),
        Day::Day22 => solution::day22::Solution::from_lines(input),
        Day::Day23 => solution::day23::Solution::from_lines(input),
        Day::Day24 => solution::day24::Solution::from_lines(input),
        Day::Day25 => solution::day25::Solution::from_lines(input),
    };

    if args.time_it {
        eprintln!(" -- Bootstrap solver: {:?}", start_time.elapsed());
        start_time = Instant::now();
    }

    let answer = match args.part {
        Part::Part1 => solver.part1(),
        Part::Part2 => solver.part2(),
    };

    if args.time_it {
        eprintln!(" -- Solution: {:?}", start_time.elapsed());
    }

    println!("{}", answer);
}
