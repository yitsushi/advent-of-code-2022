use std::str::FromStr;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Instruction(usize, usize, usize);

impl Instruction {
    pub fn count(&self) -> usize { self.0 }
    pub fn from(&self) -> usize { self.1 }
    pub fn to(&self) -> usize { self.2 }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts.len() != 6 {
            return Err(format!("unable to parse instruction: {}", s));
        }

        let count = match parts[1].parse::<usize>() {
            Ok(value) => value,
            Err(_) => return Err(format!("unable to parse instruction 'count': {}", s)),
        };

        let from = match parts[3].parse::<usize>() {
            Ok(value) => value,
            Err(_) => return Err(format!("unable to parse instruction 'from': {}", s)),
        };

        let to = match parts[5].parse::<usize>() {
            Ok(value) => value,
            Err(_) => return Err(format!("unable to parse instruction 'to': {}", s)),
        };

        Ok(Self(count, from, to))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Instruction;

    #[test]
    fn from_str() {
        let cases: Vec<(&str, Result<Instruction, String>)> = vec![
            ("move 3 from 1 to 2", Ok(Instruction(3, 1, 2))),
            ("move 3 from 1", Err("unable to parse instruction: move 3 from 1".into())),
            ("move a from 1 to 2", Err("unable to parse instruction 'count': move a from 1 to 2".into())),
            ("move 3 from a to 2", Err("unable to parse instruction 'from': move 3 from a to 2".into())),
            ("move 3 from 1 to a", Err("unable to parse instruction 'to': move 3 from 1 to a".into())),
        ];

        for case in cases {
            assert_eq!(Instruction::from_str(case.0), case.1);
        }
    }
}
