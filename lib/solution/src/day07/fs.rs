#[derive(Debug, Default, Clone)]
pub struct Directory {
    pub name: String,
    pub size: u64,
    pub files: Vec<String>,
    pub directories: Vec<Directory>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self{ name, ..Self::default() }
    }

    pub fn all_dirs(&self) -> Vec<Self> {
        let sub = self.directories
            .iter()
            .cloned()
            .flat_map(|d| d.all_dirs());

        self.directories
            .iter()
            .cloned()
            .chain(sub)
            .collect()
    }

    pub fn add_file(self, path: Vec<String>, name: String, size: u64) -> Self {
        let (files, directories) = if path.is_empty() {
            let files = self.files.iter().chain(vec![name].iter()).cloned().collect();

            (files, self.directories)
        } else {
            let (head, tail) = path.split_first().unwrap();

            let directories = self.directories
                .into_iter()
                .map(|d|
                     if &d.name != head {
                         d
                     } else {
                         d.add_file(tail.into(), name.clone(), size)
                     }
                )
                .collect();

            (self.files, directories)
        };

        Self{
            name: self.name,
            size: self.size + size,
            files,
            directories,
        }
    }

    pub fn add_dir(self, path: Vec<String>, dir: Self) -> Self {
        let directories = if path.is_empty() {
            self.directories.iter().chain(vec![dir].iter()).cloned().collect()
        } else {
            let (head, tail) = path.split_first().unwrap();

            self.directories
                .into_iter()
                .map(|d|
                     if &d.name != head {
                         d
                     } else {
                         d.add_dir(tail.into(), dir.clone())
                     }
                )
                .collect()
        };

        Self{
            name: self.name,
            size: self.size,
            files: self.files,
            directories,
        }
    }
}


#[derive(Debug)]
pub enum Entry {
    Directory(String),
    File(String, u64),
}

impl std::str::FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("invalid entry: {}", s));
        };

        if parts[0] == "dir" {
            return Ok(Self::Directory(parts[1].to_string()))
        }

        let size = match parts[0].parse::<u64>() {
            Ok(value) => value,
            Err(_) => return Err(format!("unable to parse file size: {}", s))
        };

        Ok(Self::File(parts[1].to_string(), size))
    }
}
