pub mod args;
pub mod io;

pub trait Solver {
    fn read_lines(&mut self, lines: Vec<String>);
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
#[derive(Default)]
pub struct MissingSolution {}

impl Solver for MissingSolution {
    fn part1(&self) -> String {
        todo!()
    }

    fn part2(&self) -> String {
        todo!()
    }

    fn read_lines(&mut self, _: Vec<String>) {
        todo!()
    }
}

impl MissingSolution {
    pub fn new() -> Self {
        MissingSolution{}
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::MissingSolution;

    #[test]
    #[should_panic]
    fn read_lines() {
        MissingSolution::new().read_lines(Vec::new());
    }

    #[test]
    #[should_panic]
    fn part1() {
        MissingSolution::new().part1();
    }

    #[test]
    #[should_panic]
    fn part2() {
        MissingSolution::new().part2();
    }
}
