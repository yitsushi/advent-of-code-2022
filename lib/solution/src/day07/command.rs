#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    List,
    ChDir(String),
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let cmd = parts[0];
        match cmd {
            "ls" => Ok(Command::List),
            "cd" => if parts.len() > 1 {
                Ok(Command::ChDir(parts[1].to_string()))
            } else {
                Err(format!("command error: {}", s))
            },
            _ => Err(format!("unknwon command: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Command;

    #[test]
    fn from_str() {
        let cases: Vec<(&str, Result<Command, String>)> = vec![
            ("ls", Ok(Command::List)),
            ("cd dir", Ok(Command::ChDir("dir".into()))),
            ("cd /", Ok(Command::ChDir("/".into()))),
            ("cd ..", Ok(Command::ChDir("..".into()))),
            ("cd", Err("command error: cd".into())),
            ("something", Err("unknwon command: something".into())),
        ];

        for case in cases {
            assert_eq!(Command::from_str(case.0), case.1);
        }
    }
}
