use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Day {
    Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09, Day10,
    Day11, Day12, Day13, Day14, Day15, Day16, Day17, Day18, Day19, Day20,
    Day21, Day22, Day23, Day24, Day25,
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Day01),
            "2" => Ok(Self::Day02),
            "3" => Ok(Self::Day03),
            "4" => Ok(Self::Day04),
            "5" => Ok(Self::Day05),
            "6" => Ok(Self::Day06),
            "7" => Ok(Self::Day07),
            "8" => Ok(Self::Day08),
            "9" => Ok(Self::Day09),
            "10" => Ok(Self::Day10),
            "11" => Ok(Self::Day11),
            "12" => Ok(Self::Day12),
            "13" => Ok(Self::Day13),
            "14" => Ok(Self::Day14),
            "15" => Ok(Self::Day15),
            "16" => Ok(Self::Day16),
            "17" => Ok(Self::Day17),
            "18" => Ok(Self::Day18),
            "19" => Ok(Self::Day19),
            "20" => Ok(Self::Day20),
            "21" => Ok(Self::Day21),
            "22" => Ok(Self::Day22),
            "23" => Ok(Self::Day23),
            "24" => Ok(Self::Day24),
            "25" => Ok(Self::Day25),
            _ => Err(format!("unable to parse: {}", s))
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day01 => write!(f, "01"),
            Self::Day02 => write!(f, "02"),
            Self::Day03 => write!(f, "03"),
            Self::Day04 => write!(f, "04"),
            Self::Day05 => write!(f, "05"),
            Self::Day06 => write!(f, "06"),
            Self::Day07 => write!(f, "07"),
            Self::Day08 => write!(f, "08"),
            Self::Day09 => write!(f, "09"),
            Self::Day10 => write!(f, "10"),
            Self::Day11 => write!(f, "11"),
            Self::Day12 => write!(f, "12"),
            Self::Day13 => write!(f, "13"),
            Self::Day14 => write!(f, "14"),
            Self::Day15 => write!(f, "15"),
            Self::Day16 => write!(f, "16"),
            Self::Day17 => write!(f, "17"),
            Self::Day18 => write!(f, "18"),
            Self::Day19 => write!(f, "19"),
            Self::Day20 => write!(f, "20"),
            Self::Day21 => write!(f, "21"),
            Self::Day22 => write!(f, "22"),
            Self::Day23 => write!(f, "23"),
            Self::Day24 => write!(f, "24"),
            Self::Day25 => write!(f, "25"),
        }
    }
}

