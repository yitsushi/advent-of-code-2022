use std::{str::FromStr, fmt::Display};

use super::outcome::Outcome;

#[derive(Eq, PartialEq, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
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
