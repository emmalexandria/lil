mod files;
use clap::{arg, command, Command};
use files::{get_cwd_files, File};

fn main() {
    let matches = command!()
        .arg(arg!(-a --all "List all files"))
        .arg(arg!(-A --author "Show file authors"))
        .arg(arg!(-U --unsorted "Do not sort entries"))
        .arg(arg!(-d --deref "Show file information to destination of symbolic link instead of link file"))
        .subcommand(
            Command::new("tree")
                .about("Lists files as a tree")
                .arg(arg!(-d --depth "List ").default_value("2")),
        )
        .subcommand(Command::new("long").about("List all file information in vertical list"))
        .get_matches_from(wild::args());
    let files = get_cwd_files().unwrap();

    for file in files {
        println!("{:#}", file.name)
    }
}
