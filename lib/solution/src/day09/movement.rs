#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl std::str::FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() < 2 {
            return Err(format!("unable to parse movement: '{}'", s))
        }

        let value = match parts[1].parse::<i32>() {
            Ok(v) => v,
            Err(_) => return Err(format!("unable to parse movement: '{}'", s)),
        };

        let d = match parts[0] {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => return Err(format!("unable to parse movement: '{}'", s)),
        };

        Ok(d(value))
    }
}

impl Movement {
    pub fn lift(&self) -> i32 {
        match self {
            Movement::Up(v) => *v,
            Movement::Down(v) => *v,
            Movement::Left(v) => *v,
            Movement::Right(v) => *v,
        }
    }

    pub fn split(&self) -> Vec<Movement> {
        let f = match self {
            Movement::Up(_) => Movement::Up,
            Movement::Down(_) => Movement::Down,
            Movement::Left(_) => Movement::Left,
            Movement::Right(_) => Movement::Right,
        };
        (0..self.lift())
            .into_iter()
            .map(|_| f(1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse() {
        let cases: Vec<(&str, Result<Movement, String>)> = vec![
            ("R 4", Ok(Movement::Right(4))),
            ("L 5", Ok(Movement::Left(5))),
            ("U 6", Ok(Movement::Up(6))),
            ("D 7", Ok(Movement::Down(7))),
            ("P 8", Err("unable to parse movement: 'P 8'".to_string())),
            ("Q", Err("unable to parse movement: 'Q'".to_string())),
            ("", Err("unable to parse movement: ''".to_string())),
        ];
        for case in cases {
            assert_eq!(Movement::from_str(case.0), case.1);
        }
    }

    #[test]
    fn lift() {
        assert_eq!(Movement::Up(5).lift(), 5);
        assert_eq!(Movement::Down(4).lift(), 4);
        assert_eq!(Movement::Left(8).lift(), 8);
        assert_eq!(Movement::Right(10).lift(), 10);
    }

    #[test]
    fn split() {
        assert_eq!(Movement::Up(5).split(), vec![Movement::Up(1); 5]);
        assert_eq!(Movement::Down(4).split(), vec![Movement::Down(1); 4]);
        assert_eq!(Movement::Left(2).split(), vec![Movement::Left(1); 2]);
        assert_eq!(Movement::Right(3).split(), vec![Movement::Right(1); 3]);
        assert_eq!(Movement::Right(0).split(), vec![]);
    }
}
