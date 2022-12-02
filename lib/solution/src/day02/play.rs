use std::{str::FromStr, fmt::Display};

use super::{hand::Hand, outcome::Outcome};

#[derive(Debug, PartialEq, Eq)]
pub struct Play(pub(crate) Hand, pub(crate) Outcome);

impl FromStr for Play {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(format!("invalid play: {}", s))
        }

        let opponent = Hand::from_str(parts[0])?;
        let planned = Outcome::from_str(parts[1])?;

        Ok(Play(opponent, planned))
    }
}

impl Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} against {}", self.1, self.0)
    }
}

impl Play {
    pub fn score_part1(&self) -> i32 {
        self.1.as_hand().match_against(&self.0).score().unwrap()
    }

    pub fn score(&self) -> i32 {
        match self.1.score() {
            None => self.1.played_against(&self.0).score().unwrap(),
            Some(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day02::{outcome::Outcome, hand::Hand};
    use super::Play;

    #[test]
    fn score_part1() {
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
    fn from_str() {
        assert_eq!(Play::from_str("A X"), Ok(Play(Hand::Rock, Outcome::Lose(None))));
        assert_eq!(Play::from_str("B Y"), Ok(Play(Hand::Paper, Outcome::Draw(None))));
        assert_eq!(Play::from_str("C Z"), Ok(Play(Hand::Scissors, Outcome::Win(None))));
        assert_eq!(Play::from_str("A L"), Err("invalid outcome: L".to_string()));
        assert_eq!(Play::from_str("L X"), Err("invalid move: L".to_string()));
        assert_eq!(Play::from_str("L L"), Err("invalid move: L".to_string()));
        assert_eq!(Play::from_str("LAD"), Err("invalid play: LAD".to_string()));
    }

    #[test]
    fn display() {
        let cases: Vec<(Play, String)> = vec![
            (Play::from_str("A X").unwrap(), "lose against Rock".into()),
            (Play::from_str("B Y").unwrap(), "draw against Paper".into()),
            (Play::from_str("C Z").unwrap(), "win against Scissors".into()),
        ];

        for (input, output) in cases {
            assert_eq!(format!("{}", input), output);
        }
    }

    #[test]
    fn score() {
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

            (Play(Hand::Scissors, Outcome::Win(Some(Hand::Rock))), 7),  // Rock
        ];

        for (play, score) in plays {
            assert_eq!(play.score(), score, "{} should be {}, but we have {}", play, score, play.score());
        }
    }
}
