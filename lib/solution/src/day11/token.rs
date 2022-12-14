#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Value(i64),
    Old,
    Add,
    Mul,
}

impl std::str::FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Token::Old),
            "+" => Ok(Token::Add),
            "*" => Ok(Token::Mul),
            word => {
                if let Ok(v) = word.parse::<i64>() {
                    Ok(Token::Value(v))
                } else {
                    Err(format!("unable to parse optoken: {}", s))
                }
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Operation {
    ops: Vec<Token>
}

impl Operation {
    pub fn new(ops: Vec<Token>) -> Self {
        Self { ops }
    }

    pub fn fold(self, init: i64, modi: i64) -> i64 {
        let (value, _) = self.ops.iter().fold((0, None::<Token>), |(value, prev), op| {
            match op {
                Token::Value(v) => match prev {
                    Some(Token::Mul) => (value * v % modi, None),
                    _ => (value + v, None),
                },
                Token::Old => match prev {
                    Some(Token::Mul) => (value * init % modi, None),
                    _ => (value + init, None),
                },
                Token::Add | Token::Mul => (value, Some(op.clone())),
            }
        });

        value
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse() {
        let cases: Vec<(&str, Result<Token, String>)> = vec![
            ("13", Ok(Token::Value(13))),
            ("old", Ok(Token::Old)),
            ("+", Ok(Token::Add)),
            ("*", Ok(Token::Mul)),
            ("asd", Err("unable to parse optoken: asd".to_string())),
        ];

        for case in cases {
            assert_eq!(Token::from_str(case.0), case.1);
        }
    }

    #[test]
    fn fold_opeartions() {
        let cases: Vec<(i64, Vec<Token>, i64)> = vec![
            // old + 8
            (3, vec![Token::Old, Token::Add, Token::Value(8)], 11),
            // old * 3
            (5, vec![Token::Old, Token::Mul, Token::Value(3)], 15),
            // old * old
            (5, vec![Token::Old, Token::Mul, Token::Old], 25),
            // 5 * old + 2
            (5, vec![Token::Old, Token::Mul, Token::Old, Token::Add, Token::Value(2)], 27),
        ];

        for case in cases {
            let op = Operation::new(case.1);
            assert_eq!(op.fold(case.0, 100), case.2);
        }
    }
}
