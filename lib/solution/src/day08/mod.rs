use std::collections::HashSet;

mod forest;

/// Day 8: Treetop Tree House
///
/// Consider your map; how many trees are visible from outside the grid?
///
/// URL: <https://adventofcode.com/2022/day/8>
///
/// # Part 1
///
/// ```
/// use aoc::Solver;
///
/// let lines: Vec<&str> = vec![
///   "30373",
///   "25512",
///   "65332",
///   "33549",
///   "35390",
/// ];
///
/// let mut solver = solution::day08::Solution::new();
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
///   "30373",
///   "25512",
///   "65332",
///   "33549",
///   "35390",
/// ];
///
/// let mut solver = solution::day08::Solution::new();
/// solver.read_lines(lines.iter().map(|s| s.to_string()).collect());
///
/// let part2_solution = solver.part2();
/// ```
#[derive(Default)]
pub struct Solution {
    forest: forest::Forest,
}

impl aoc::Solver for Solution {
    fn name(&self) -> &'static str {
        "day08::Solution"
    }

    fn read_lines(&mut self, lines: Vec<String>) {
        let rows = lines.len();
        let columns = if let Some(line) = lines.first() {
            line.len()
        } else {
            return
        };

        let trees = lines
            .iter()
            .flat_map(|line| {
                line.clone().into_bytes()
                    .iter()
                    .map(|ch| ch - 0x30)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>()
            ;

        self.forest = forest::Forest::new(trees, rows, columns)
    }

    fn part1(&mut self) -> String {
        let mut visible: HashSet<usize> = HashSet::new();

        let rows = self.forest.all_rows();
        let cols = self.forest.all_columns();

        let check = rows.iter()
            .chain(cols.iter());

        for current in check {
            self.list_visible_trees(current, &mut visible);

            let mut rev = current.clone();
            rev.reverse();
            self.list_visible_trees(&rev, &mut visible);
        }

        let mut visible_trees = visible.iter()
            .map(|v| (*v, self.forest.at(*v).unwrap()))
            .collect::<Vec<(usize, u8)>>();

        visible_trees.sort_by(|a, b| a.0.cmp(&b.0));

        format!("{}", visible.len())
    }

    fn part2(&mut self) -> String {
        let score = self.forest.all().iter()
            .enumerate()
            .map(|(idx, _)| self.forest.scenic_score_at(idx) )
            .max()
            ;

        format!("{}", score.unwrap())
    }
}

impl Solution {
    /// New empty solution.
    pub fn new() -> Self { Self::default() }

    fn list_visible_trees(&self, list: &[(usize, u8)], visible: &mut HashSet<usize>) {
        let mut highest: Option<u8> = None;

        for (idx, tree) in list.iter() {
            highest = match highest {
                Some(v) if tree <= &v => continue,
                Some(_) => Some(*tree),
                None => Some(*tree),
            };

            visible.insert(*idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::io::{Filesystem, LocalFilesystem};
    use aoc::Solver;

    use super::Solution;

    #[test]
    fn name() {
        assert_eq!(Solution::new().name(), "day08::Solution")
    }

    #[test]
    fn example1_part1() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day08").unwrap());

        assert_eq!(solver.part1(), format!("{}", 21));
    }

    #[test]
    fn example1_part2() {
        let fs = LocalFilesystem{};
        let mut solver = Solution::new();
        solver.read_lines(fs.read_file("tests/fixtures/day08").unwrap());

        assert_eq!(solver.part2(), format!("{}", 8));
    }
}
