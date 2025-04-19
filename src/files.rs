use std::{fs, io::Error, os};

pub struct File {
    pub name: String,
}

pub fn get_cwd_files() -> Result<Vec<File>, Error> {
    let cwd = std::env::current_dir()?;

    let read_dir = fs::read_dir(cwd)?;
    let mut files = vec![];
    for file in read_dir {
        let f = file?;
        files.push(File {
            name: f.file_name().into_string().unwrap(),
        });
    }

    Ok(files)
}
