use std::{str::FromStr, fmt::Display};

use super::{hand::Hand, outcome::Outcome};

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
