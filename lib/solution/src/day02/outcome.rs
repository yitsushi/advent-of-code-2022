use std::{str::FromStr, fmt::Display};

use super::hand::Hand;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Lose(Option<Hand>),
    Draw(Option<Hand>),
    Win(Option<Hand>),
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose(None)),
            "Y" => Ok(Outcome::Draw(None)),
            "Z" => Ok(Outcome::Win(None)),
            _ => Err(format!("invalid outcome: {}", s))
        }
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Outcome::Lose(_) => write!(f, "lose"),
            Outcome::Draw(_) => write!(f, "draw"),
            Outcome::Win(_) => write!(f, "win"),
        }
    }
}

impl Outcome {
    pub fn played_against(&self, other: &Hand) -> Self {
        match self {
            Outcome::Lose(None) => Outcome::Lose(Some(other.stronger_than())),
            Outcome::Draw(None) => Outcome::Draw(Some(other.clone())),
            Outcome::Win(None) => Outcome::Win(Some(other.weaker_than())),
            _ => self.clone(),
        }
    }

    pub fn score(&self) -> Option<i32> {
        match self {
            Outcome::Lose(None) => None,
            Outcome::Draw(None) => None,
            Outcome::Win(None) => None,

            Outcome::Lose(Some(hand)) => Some(hand.score()),
            Outcome::Draw(Some(hand)) => Some(hand.score() + 3),
            Outcome::Win(Some(hand)) => Some(hand.score() + 6),
        }
    }

    pub fn as_hand(&self) -> Hand {
        match self {
            Outcome::Lose(_) => Hand::Rock,
            Outcome::Draw(_) => Hand::Paper,
            Outcome::Win(_) => Hand::Scissors,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day02::hand::Hand;

    use super::Outcome;

    #[test]
    fn as_hand() {
        let cases: Vec<(Outcome, Hand)> = vec![
            (Outcome::Lose(None), Hand::Rock),
            (Outcome::Draw(None), Hand::Paper),
            (Outcome::Win(None), Hand::Scissors),
        ];

        for (input, output) in cases {
            assert_eq!(input.as_hand(), output);
        }
    }

    #[test]
    fn played_against() {
        assert_eq!(Outcome::Lose(None).played_against(&Hand::Rock), Outcome::Lose(Some(Hand::Scissors)));
        assert_eq!(Outcome::Win(None).played_against(&Hand::Rock), Outcome::Win(Some(Hand::Paper)));
        assert_eq!(Outcome::Draw(None).played_against(&Hand::Rock), Outcome::Draw(Some(Hand::Rock)));

        assert_eq!(Outcome::Lose(Some(Hand::Scissors)).played_against(&Hand::Rock), Outcome::Lose(Some(Hand::Scissors)));
    }

    #[test]
    fn score() {
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
    fn from_str() {
        assert_eq!(Outcome::from_str("X"), Ok(Outcome::Lose(None)));
        assert_eq!(Outcome::from_str("Y"), Ok(Outcome::Draw(None)));
        assert_eq!(Outcome::from_str("Z"), Ok(Outcome::Win(None)));
        assert_eq!(Outcome::from_str("L"), Err("invalid outcome: L".to_string()));
    }

    #[test]
    fn display() {
        let cases: Vec<(Outcome, String)> = vec![
            (Outcome::Lose(None), "lose".into()),
            (Outcome::Draw(None), "draw".into()),
            (Outcome::Win(None), "win".into()),
        ];

        for (input, output) in cases {
            assert_eq!(format!("{}", input), output);
        }
    }
}
