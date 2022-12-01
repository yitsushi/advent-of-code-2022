use std::{io::{BufRead, BufReader}, fs::File, fmt::Display};

#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    ReadError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError(file) => write!(f, "unable to read file: {}", file),
        }
    }
}

pub fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let reader = match File::open(filename) {
        Ok(file) => BufReader::new(file),
        Err(_) => return Err(Error::ReadError(filename.to_string())),
    };

    Ok(reader.lines().filter_map(std::io::Result::ok).collect())
}

#[cfg(test)]
mod tests {
    use super::{read_input, Error};

    #[test]
    fn no_error() {
        let filename = "tests/fixtures/valid_file";
        match read_input(filename) {
            Ok(input) => {
                assert_eq!(input, vec!["something".to_string()])
            },
            Err(err) => {
                panic!("{}", err);
            },
        }
    }

    #[test]
    fn file_not_found() {
        let filename = "__file_is_not_here__";
        match read_input(filename) {
            Ok(_) => {
                panic!("file should not exist: {}", filename)
            },
            Err(err) => {
                assert_eq!(err, Error::ReadError(filename.to_string()))
            },
        }
    }
}
