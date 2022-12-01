use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Part {
    Part1,
    Part2,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Part1),
            "2" => Ok(Self::Part2),
            _ => Err(format!("unable to parse: {}", s))
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Part1 => write!(f, "1"),
            Self::Part2 => write!(f, "2"),
        }
    }
}
