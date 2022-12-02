use std::str::FromStr;

use self::play::Play;

mod play;
mod hand;
mod outcome;

pub struct Solution {
    plays: Vec<Play>
}

impl aoc::Solution for Solution {
    fn part1(&self) -> String {
        let score: i32 = self.plays
            .iter()
            .map(|play| play.score_part1() )
            .sum();

        score.to_string()
    }

    fn part2(&self) -> String {
        let score: i32 = self.plays
            .iter()
            .map(|play| play.score() )
            .sum();

        score.to_string()
    }
}

impl Solution {
    pub fn from_lines(lines: Vec<String>) -> Box<dyn aoc::Solution> {
        let plays: Vec<Play> = lines
            .iter()
            .map(|s| { Play::from_str(s).expect("input parse error") })
            .collect();

        Box::new(Solution{plays})
    }
}


#[cfg(test)]
mod tests {
    use super::{Solution, hand::Hand, play::Play, outcome::Outcome};

    fn input(filename: &str) -> Vec<String> {
        match aoc::io::read_input(filename) {
            Ok(inp) => inp,
            Err(err) => { panic!("Error: {}", err); },
        }
    }

    #[test]
    fn hand_score() {
        assert_eq!(Hand::Rock.score(), 1);
        assert_eq!(Hand::Paper.score(), 2);
        assert_eq!(Hand::Scissors.score(), 3);
    }

    #[test]
    fn play_score_part1() {
        let plays: Vec<(Play, i32)> = vec![
            (Play(Hand::Rock, Outcome::Lose(None)), 4),
            (Play(Hand::Rock, Outcome::Draw(None)), 8),
            (Play(Hand::Rock, Outcome::Win(None)), 3),

            (Play(Hand::Paper, Outcome::Lose(None)), 1),
            (Play(Hand::Paper, Outcome::Draw(None)), 5),
            (Play(Hand::Paper, Outcome::Win(None)), 9),

            (Play(Hand::Scissors, Outcome::Lose(None)), 7),
            (Play(Hand::Scissors, Outcome::Draw(None)), 2),
            (Play(Hand::Scissors, Outcome::Win(None)), 6),
        ];

        for (play, score) in plays {
            assert_eq!(play.score_part1(), score, "{} should be {}, but we have {}", play, score, play.score());
        }
    }

    #[test]
    fn play_score() {
        let plays: Vec<(Play, i32)> = vec![
            (Play(Hand::Rock, Outcome::Lose(None)), 3),     // Scissors
            (Play(Hand::Rock, Outcome::Draw(None)), 4),     // Rock
            (Play(Hand::Rock, Outcome::Win(None)), 8),      // Paper

            (Play(Hand::Paper, Outcome::Lose(None)), 1),    // Rock
            (Play(Hand::Paper, Outcome::Draw(None)), 5),    // Paper
            (Play(Hand::Paper, Outcome::Win(None)), 9),     // Scissors

            (Play(Hand::Scissors, Outcome::Lose(None)), 2), // Paper
            (Play(Hand::Scissors, Outcome::Draw(None)), 6), // Scissors
            (Play(Hand::Scissors, Outcome::Win(None)), 7),  // Rock
        ];

        for (play, score) in plays {
            assert_eq!(play.score(), score, "{} should be {}, but we have {}", play, score, play.score());
        }
    }

    #[test]
    fn outcome() {
        let outcomes: Vec<(Outcome, Option<i32>, &str)> = vec![
            (Outcome::Lose(None), None, "unknown lose"),
            (Outcome::Draw(None), None, "unknown draw"),
            (Outcome::Win(None), None, "unknown win"),

            (Outcome::Lose(Some(Hand::Rock)), Some(1), "lose with rock"),
            (Outcome::Lose(Some(Hand::Paper)), Some(2), "lose with paper"),
            (Outcome::Lose(Some(Hand::Scissors)), Some(3), "lose with scissors"),

            (Outcome::Draw(Some(Hand::Rock)), Some(4), "draw with rock"),
            (Outcome::Draw(Some(Hand::Paper)), Some(5), "draw with rock"),
            (Outcome::Draw(Some(Hand::Scissors)), Some(6), "draw with rock"),

            (Outcome::Win(Some(Hand::Rock)), Some(7), "win with rock"),
            (Outcome::Win(Some(Hand::Paper)), Some(8), "win with rock"),
            (Outcome::Win(Some(Hand::Scissors)), Some(9), "win with rock"),
        ];

        for (outcome, score, case) in outcomes {
            assert_eq!(outcome.score(), score, "{}", case);
        }
    }

    #[test]
    fn example1() {
        let solver = Solution::from_lines(input("tests/fixtures/day02"));

        assert_eq!(solver.part1(), format!("{}", 15));
        assert_eq!(solver.part2(), format!("{}", 12));
    }
}
