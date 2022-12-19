use aoc::{math::{Vec2D, abs}, range::Range};

mod grid;

/// Day 15: Beacon Exclusion Zone
///
/// Consult the report from the sensors you just deployed. In the row where y=2000000, how many
/// positions cannot contain a beacon?
///
/// URL: <https://adventofcode.com/2022/day/15>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "Sensor at x=3842919, y=126080: closest beacon is at x=3943893, y=1918172",
///   "Sensor at x=406527, y=2094318: closest beacon is at x=-1066, y=1333278",
///   "Sensor at x=2208821, y=3683408: closest beacon is at x=2914373, y=3062268",
///   "Sensor at x=39441, y=1251806: closest beacon is at x=-1066, y=1333278",
///   "Sensor at x=3093352, y=2404566: closest beacon is at x=2810772, y=2699609",
///   "Sensor at x=3645473, y=2234498: closest beacon is at x=3943893, y=1918172",
/// ];
///
/// let mut solver = solution::day15::Solution::new();
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
///   "Sensor at x=3842919, y=126080: closest beacon is at x=3943893, y=1918172",
///   "Sensor at x=406527, y=2094318: closest beacon is at x=-1066, y=1333278",
///   "Sensor at x=2208821, y=3683408: closest beacon is at x=2914373, y=3062268",
///   "Sensor at x=39441, y=1251806: closest beacon is at x=-1066, y=1333278",
///   "Sensor at x=3093352, y=2404566: closest beacon is at x=2810772, y=2699609",
///   "Sensor at x=3645473, y=2234498: closest beacon is at x=3943893, y=1918172",
/// ];
///
/// let mut solver = solution::day15::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    grid: grid::Grid,
    target_line: i64,
    max_point: i64,
}

fn parse_i64(s: &str) -> i64 {
    s.trim_matches(|f| f != '-' && !char::is_numeric(f))
        .parse::<i64>()
        .unwrap()
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day15::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        lines.into_iter()
            .for_each(|line| {
                let parts = line.split(' ').collect::<Vec<&str>>();
                let probe_x = parse_i64(parts.get(2).unwrap());
                let probe_y = parse_i64(parts.get(3).unwrap());
                let beacon_x = parse_i64(parts.get(8).unwrap());
                let beacon_y = parse_i64(parts.get(9).unwrap());

                self.grid.add_probe(Vec2D::new(probe_x, probe_y), Vec2D::new(beacon_x, beacon_y));
            });

        self.target_line = 2000000;
        self.max_point = 4000000;
    }

    fn part1(&mut self) -> String {
        let ranges = self.grid.probes.iter()
            .filter_map(|probe| {
                let y_distance = abs(probe.position.y - self.target_line);
                if y_distance > probe.radius { return None }

                let min = probe.position.x - (probe.radius - y_distance);
                let max = probe.position.x + (probe.radius - y_distance);

                Some(Range::new(min, max))
            }).collect::<Vec<Range<i64>>>();
        let range = ranges.iter().fold(ranges.get(0).unwrap().clone(), |c, r| {
            let min = if r.from() < c.from() { r.from() } else { c.from() };
            let max = if r.to() > c.to() { r.to() } else { c.to() };

            Range::new(min, max)
        });

        let count = (range.from()..=range.to()).map(|x| {
            match self.grid.get(Vec2D::new(x, self.target_line)) {
                grid::Cell::Empty => 1,
                _ => 0,
            }
        }).sum::<i64>();

        format!("{}", count)
    }

    fn part2(&mut self) -> String {
        for y in 0..=self.max_point {
            let mut x = 0;
            loop {
                let current = Vec2D::new(x, y);
                match self.check_position(&current) {
                    None => {
                        return format!("{}", current.x * 4000000 + current.y)
                    },
                    Some(probe) => {
                        let d = abs(current.y - probe.position.y);
                        x = probe.position.x + (probe.radius - d) + 1;
                    }
                }

                if x > self.max_point {
                    break
                }
            }
        }

        format!("{}", 0)
    }
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self { Self::default() }

    fn check_position(&self, current: &Vec2D<i64>) -> Option<grid::Probe> {
        for probe in self.grid.probes.iter() {
            if probe.radius >= current.manhattan_to(&probe.position) {
                return Some(probe.clone())
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day15::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day15").unwrap());

        solver.target_line = 10;

        assert_eq!(solver.part1(), format!("{}", 26));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day15").unwrap());
        solver.max_point = 20;

        assert_eq!(solver.part2(), format!("{}", 56000011));
    }
}
