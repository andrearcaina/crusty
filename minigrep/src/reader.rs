use std::fs;
use std::io;

pub struct Reader {
    pub path: String,
}

impl Reader {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn read(&self) -> Result<String, io::Error> {
        fs::read_to_string(&self.path)
    }
}
