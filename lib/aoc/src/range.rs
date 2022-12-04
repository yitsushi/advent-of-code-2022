use std::str::FromStr;
use std::cmp::PartialOrd;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Range<T>(T, T);

impl<T: PartialOrd + FromStr> FromStr for Range<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(format!("invalid range: {}", s));
        }

        let from = match parts[0].parse::<T>() {
            Ok(v) => v,
            Err(_) => return Err(format!("invalid range: {}", s)),
        };
        let to = match parts[1].parse::<T>() {
            Ok(v) => v,
            Err(_) => return Err(format!("invalid range: {}", s)),
        };

        if from > to {
            Ok(Self(to, from))
        } else {
            Ok(Self(from, to))
        }
    }
}

impl Range<i32> {
    pub fn new(from: i32, to: i32) -> Self {
        Self(from, to)
    }

    pub fn from(&self) -> i32 {
        self.0
    }

    pub fn to(&self) -> i32 {
        self.1
    }

    pub fn size(&self) -> i32 {
        1 + self.to() - self.from()
    }

    pub fn contains(&self, other: &Range<i32>) -> bool {
        self.from() <= other.from() && self.to() >= other.to()
    }

    pub fn overlap(&self, other: &Range<i32>) -> bool {
        (self.to() >= other.from() && self.from() < other.from())
            || (self.from() <= other.to() && self.to() >= other.to())
            || self.contains(other) || other.contains(self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::Range;

    #[test]
    fn parse() {
        let cases: Vec<(&str, Result<Range<i32>,String>)> = vec![
            ("2-4", Ok(Range::new(2, 4))),
            ("6-6", Ok(Range::new(6, 6))),
            ("2-8", Ok(Range::new(2, 8))),
            ("8-2", Ok(Range::new(2, 8))),
            ("a-8", Err(format!("invalid range: a-8"))),
            ("8-c", Err(format!("invalid range: 8-c"))),
            ("1-3-4", Err(format!("invalid range: 1-3-4"))),
            ("123", Err(format!("invalid range: 123"))),
        ];

        for case in cases {
            assert_eq!(Range::from_str(case.0), case.1)
        }
    }

    #[test]
    fn size() {
        let cases: Vec<(&str, i32)> = vec![
            ("2-4", 3),
            ("6-6", 1),
            ("2-8", 7),
        ];

        for case in cases {
            let range = Range::from_str(case.0).expect("failed to parse case");
            assert_eq!(range.size(), case.1)
        }
    }

    #[test]
    fn contains() {
        let cases: Vec<(&str, &str, bool)> = vec![
            ("2-4", "1-5", false),
            ("1-5", "2-4", true),
        ];

        for case in cases {
            let range1 = Range::from_str(case.0).expect("failed to parse case");
            let range2 = Range::from_str(case.1).expect("failed to parse case");
            assert_eq!(range1.contains(&range2), case.2, "({}).contains({}) should be {}", case.0, case.1, case.2);
        }
    }

    #[test]
    fn overlap() {
        let cases: Vec<(&str, &str, bool)> = vec![
            ("2-4", "6-8", false),
            ("4-5", "2-3", false),
            ("5-7", "7-9", true),
            ("6-6", "4-6", true),
            ("1-5", "2-4", true),
            ("2-4", "1-5", true),
        ];

        for case in cases {
            let range1 = Range::from_str(case.0).expect("failed to parse case");
            let range2 = Range::from_str(case.1).expect("failed to parse case");
            assert_eq!(range1.overlap(&range2), case.2, "({}).overlap({}) should be {}", case.0, case.1, case.2);
        }
    }
}