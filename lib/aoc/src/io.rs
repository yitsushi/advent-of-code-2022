use std::{io::{BufRead, BufReader}, fs::File, fmt::Display};

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
