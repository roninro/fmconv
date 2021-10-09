use clap::{crate_version, App, Arg};
use std::{ path::Path, str::FromStr};
use walkdir::{DirEntry, WalkDir};

mod errors;
mod format;

use errors::Error;
use format::Format;

fn main() {
    let matches = App::new("Front-matter converter")
        .version(crate_version!())
        .arg(
            Arg::new("format")
                .short('f')
                .about("yaml/toml format")
                .default_value("yaml"),
        )
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .multiple_values(true)
                .required(true),
        )
        .get_matches();

    let paths: Vec<&str> = matches.values_of("INPUT").unwrap().collect();

    let f = matches.value_of("format").unwrap();

    if let Err(e) = run(f, paths) {
        handle_error(e)
    }
}

fn handle_error(error: Error) {
    println!("{}", error);
}

fn run(f: &str, paths: Vec<&str>) -> Result<(), Error> {
    let fr = Format::from_str(f)?;

    for p in paths.into_iter() {
        for entry in WalkDir::new(p) {
            match entry {
                Ok(entry) => parse_file(entry.path(), fr),
                Err(e) => handle_error(Error::from(e)),
            }
        }
    }

    Ok(())
}

fn parse_file(path: &Path, fr: Format) {
    println!("{} {:?}", path.display(), path.extension())
}
