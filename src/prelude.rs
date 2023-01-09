pub use crate::password_file::*;
pub use crate::pw_entry::*;
pub use inquire::{Password, Select, Text};
pub use lazy_static::lazy_static;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use std::fs::{create_dir, read_dir};

pub fn open_file() -> Result<PasswordFile> {
    let dir = match read_dir("password_data") {
        Ok(f) => f,
        Err(_) => {
            create_dir("password_data")?;
            read_dir("password_data")?
        }
    };

    let mut options = Vec::<String>::new();
    for file in dir {
        let Ok(entry) = file else {
            break
        };

        let name = entry
            .file_name()
            .to_str()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()[0]
            .to_owned();
        options.push(name);
    }

    if options.len() == 0 {
        let path = new_file()?;
        let pwf = PasswordFile::new(path)?;
        return Ok(pwf);
    }

    let choice = Select::new("Select the file you wish to open:", options).prompt()?;
    let path = format!("password_data/{}.pwf", choice);
    Ok(PasswordFile::open(path)?)
}

fn new_file() -> Result<String> {
    let name = Text::new("Enter the name for the new file:").prompt()?;
    Ok(format!("password_data/{}.pwf", name))
}
