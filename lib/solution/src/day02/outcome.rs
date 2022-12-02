use std::{str::FromStr, fmt::Display};

use super::hand::Hand;

#[derive(Clone)]
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
