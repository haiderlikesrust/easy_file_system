use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;

pub struct FileHandler {
    /// Enter path with the file name
    pub path: String,
    /// Enter ONLY filename with its extension
    pub file_name: String,
}

impl FileHandler {
    /// Reads the file and returns a `Result` Type
    pub fn read(&self) -> io::Result<String> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        Ok(buffer)
    }

    /// Creates a new `FileHandler`
    pub fn new(path: &str, file_name: &str) -> Result<Self, io::Error> {
        let file = File::create(path);
        match file {
            Ok(_) => {
                Ok(Self { path: path.to_string(), file_name: file_name.to_owned() })
            },
            Err(e) => Err(e)
        }
    }

    /// Writes into the file and returns a `Result` Type
    pub fn write(&self, content: &str) -> io::Result<()> {
        let file = OpenOptions::new().write(true).open(&self.path)?;
        let mut writer = BufWriter::new(file);
        writer.write(content.as_bytes())?;
        writer.flush()

    }

    pub fn extension(&self) {
        let f: Vec<&str> = self.file_name.split('.').collect();
        println!("{:?}", f)
    }

    pub fn keep_content_and_write(&self, content: &str) -> io::Result<()> {
        let p_content = self.read()?;
        let joined_content = format!("{}\n {}", p_content, content);
        self.write(&joined_content)?;
        Ok(())
    }

}
