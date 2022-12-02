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

pub trait Filesystem {
    fn path_exists(&self, path: &str) -> bool;
    fn read_file(&self, path: &str) -> Result<Vec<String>, Error>;
}

pub struct LocalFilesystem {}

impl Filesystem for LocalFilesystem {
    fn path_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn read_file(&self, path: &str) -> Result<Vec<String>, Error> {
        let reader = match File::open(path) {
            Ok(file) => BufReader::new(file),
            Err(_) => return Err(Error::ReadError(path.to_string())),
        };

        Ok(reader.lines().filter_map(std::io::Result::ok).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{Filesystem, LocalFilesystem, Error};

    #[test]
    fn no_error() {
        let fs = LocalFilesystem{};

        let filename = "tests/fixtures/valid_file";
        match fs.read_file(filename) {
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
        let fs = LocalFilesystem{};

        let filename = "__file_is_not_here__";
        match fs.read_file(filename) {
            Ok(_) => {
                panic!("file should not exist: {}", filename);
            },
            Err(err) => {
                assert_eq!(err, Error::ReadError(filename.to_string()));
                assert_eq!(format!("{}", err), format!("unable to read file: {}", filename))
            },
        }
    }
}
