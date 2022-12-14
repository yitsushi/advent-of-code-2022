#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    NoOp,
    Addx(i32),
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::NoOp)
        }

        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() >= 2 && "addx" == parts[0] {
            if let Ok(v) = parts[1].parse::<i32>() {
                return Ok(Instruction::Addx(v))
            }
        }

        Err(format!("unable to parse instruction: {}", s))
    }
}

impl Instruction {
    pub fn cycles(&self) -> i32 {
        match self {
            Instruction::NoOp => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse() {
        let cases: Vec<(&str, Result<Instruction, String>)> = vec![
            ("noop", Ok(Instruction::NoOp)),
            ("addx 10", Ok(Instruction::Addx(10))),
            ("addx -3", Ok(Instruction::Addx(-3))),
            ("something", Err("unable to parse instruction: something".to_string())),
            ("addx", Err("unable to parse instruction: addx".to_string())),
            ("addx asd", Err("unable to parse instruction: addx asd".to_string())),
        ];

        for case in cases {
            assert_eq!(Instruction::from_str(case.0), case.1);
        }
    }
}
