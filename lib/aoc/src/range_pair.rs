use std::str::FromStr;
use crate::range::Range;
use num::PrimInt;

#[derive(Debug, PartialEq, Eq)]
pub struct RangePair<T>(Range<T>, Range<T>);

impl<T: PartialOrd + FromStr> FromStr for RangePair<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(format!("invalid pair: {}", s));
        }

        let fst = match parts[0].parse::<Range<T>>() {
            Ok(v) => v,
            Err(err) => return Err(format!("invalid pair: {}", err)),
        };
        let snd = match parts[1].parse::<Range<T>>() {
            Ok(v) => v,
            Err(err) => return Err(format!("invalid pair: {}", err)),
        };

        Ok(Self(fst, snd))
    }
}

impl<T: PrimInt> RangePair<T> {
    pub fn fst(&self) -> &Range<T> {
        &self.0
    }

    pub fn snd(&self) -> &Range<T> {
        &self.1
    }

    pub fn useless(&self) -> Option<Range<T>> {
        if self.fst().contains(self.snd()) {
            return Some(self.snd().clone())
        }

        if self.snd().contains(self.fst()) {
            return Some(self.fst().clone())
        }

        None
    }

    pub fn overlap(&self) -> bool {
        self.fst().overlap(self.snd())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::range::Range;
    use super::RangePair;

    #[test]
    fn parse() {
        let cases: Vec<(&str, Result<RangePair<i32>,String>)> = vec![
            ("2-4,6-8", Ok(RangePair(Range::new(2, 4), Range::new(6, 8)))),
            ("2-3,4-5", Ok(RangePair(Range::new(2, 3), Range::new(4, 5)))),
            ("6-6,4-6", Ok(RangePair(Range::new(6, 6), Range::new(4, 6)))),
            ("8-2,2-8", Ok(RangePair(Range::new(2, 8), Range::new(2, 8)))),
            ("a-8,1-3", Err(format!("invalid pair: invalid range: a-8"))),
            ("8-c,1-3", Err(format!("invalid pair: invalid range: 8-c"))),
            ("2-4", Err(format!("invalid pair: 2-4"))),
            ("1-3-4", Err(format!("invalid pair: 1-3-4"))),
            ("123", Err(format!("invalid pair: 123"))),
        ];

        for case in cases {
            assert_eq!(RangePair::from_str(case.0), case.1)
        }
    }

    #[test]
    fn parts() {
        let cases: Vec<(&str, Range<i32>, Range<i32>)> = vec![
            ("2-4,6-8", Range::new(2, 4), Range::new(6, 8)),
            ("10-4,6-8", Range::new(4, 10), Range::new(6, 8)),
        ];

        for case in cases {
            let pair = RangePair::from_str(case.0).expect("failed to parse case");
            assert_eq!(pair.fst(), &case.1);
            assert_eq!(pair.snd(), &case.2);
        }
    }

    #[test]
    fn useless() {
        let cases: Vec<(&str, Option<Range<i32>>)> = vec![
            ("1-8,3-4", Some(Range::new(3, 4))),
            ("3-4,1-8", Some(Range::new(3, 4))),
            ("2-4,3-8", None),
            ("2-4,5-8", None),
            ("3-8,2-4", None),
            ("5-8,2-4", None),
        ];

        for case in cases {
            let pair: RangePair<i32> = RangePair::from_str(case.0).expect("failed to parse case");
            assert_eq!(pair.useless(), case.1);
        }
    }

    #[test]
    fn overlap() {
        let cases: Vec<(&str, bool)> = vec![
            ("2-4,6-8", false),
            ("2-3,4-5", false),
            ("5-7,7-9", true),
            ("2-8,3-7", true),
            ("6-6,4-6", true),
            ("2-6,4-8", true),
        ];

        for case in cases {
            let pair: RangePair<i32> = RangePair::from_str(case.0).expect("failed to parse case");
            assert_eq!(pair.overlap(), case.1, "({:?}).overlap({:?}) should be {}", pair.fst(), pair.snd(), case.1);
        }
    }
}