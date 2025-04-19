mod files;
use files::{get_cwd_files, File};

fn main() {
    let files = get_cwd_files().unwrap();

    for file in files {
        println!("{:#}", file.name)
    }
}
