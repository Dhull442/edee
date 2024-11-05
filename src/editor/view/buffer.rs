use std::{fs::read_to_string, io::Error};

pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Buffer {
    pub fn load(filename: String) -> Result<Self, Error> {
        let file_contents: String = read_to_string(filename)?;
        let mut lines: Vec<String> = Vec::new();
        for line in file_contents.lines() {
            lines.push(String::from(line));
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
