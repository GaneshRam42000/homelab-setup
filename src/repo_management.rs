use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn read_or_create_repo_config(path: &str, default_content: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;

            file.write_all(default_content.as_bytes())?;

            Ok(default_content.to_string())
        }
        Err(err) => Err(err),
    }
}
