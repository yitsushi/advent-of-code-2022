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
            _ => Err(format!("unable to parse part: {}", s))
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Part;

    #[test]
    fn from_str() {
        assert_eq!(Part::from_str("1"), Ok(Part::Part1));
        assert_eq!(Part::from_str("2"), Ok(Part::Part2));
        assert_eq!(Part::from_str("3"), Err("unable to parse part: 3".into()));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Part::Part1), "1");
        assert_eq!(format!("{}", Part::Part2), "2");

    }
}
