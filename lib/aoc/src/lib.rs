pub mod args;
pub mod io;

pub trait Solution {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
