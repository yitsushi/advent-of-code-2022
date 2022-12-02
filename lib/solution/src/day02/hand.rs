use std::{str::FromStr, fmt::Display};

use super::outcome::Outcome;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err(format!("invalid move: {}", s))
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hand::Rock => write!(f, "Rock"),
            Hand::Paper => write!(f, "Paper"),
            Hand::Scissors => write!(f, "Scissors"),
        }
    }
}

impl Hand {
    pub fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn stronger_than(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    pub fn weaker_than(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    pub fn match_against(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::Draw(Some(self.clone()))
        }

        if self.stronger_than() == *other {
            Outcome::Win(Some(self.clone()))
        } else {
            Outcome::Lose(Some(self.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Hand;

    #[test]
    fn score() {
        assert_eq!(Hand::Rock.score(), 1);
        assert_eq!(Hand::Paper.score(), 2);
        assert_eq!(Hand::Scissors.score(), 3);
    }

    #[test]
    fn from_str() {
        assert_eq!(Hand::from_str("A"), Ok(Hand::Rock));
        assert_eq!(Hand::from_str("B"), Ok(Hand::Paper));
        assert_eq!(Hand::from_str("C"), Ok(Hand::Scissors));
        assert_eq!(Hand::from_str("L"), Err("invalid move: L".to_string()));
    }

    #[test]
    fn display() {
        let cases: Vec<(Hand, String)> = vec![
            (Hand::Rock, "Rock".into()),
            (Hand::Paper, "Paper".into()),
            (Hand::Scissors, "Scissors".into()),
        ];

        for (input, output) in cases {
            assert_eq!(format!("{}", input), output);
        }
    }
}
