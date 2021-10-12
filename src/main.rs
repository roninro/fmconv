use clap::{crate_version, App, Arg};
use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};
use walkdir::{DirEntry, WalkDir};

mod errors;
mod format;

use errors::Error;
use format::*;

const TOML_DELIM: &'static str = "+++";
const YAML_DELIM: &'static str = "---";

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

fn run(f: &str, paths: Vec<&str>) -> Result<(), Error> {
    let fr = Format::from_str(f)?;
    for p in paths.into_iter() {
        for entry in WalkDir::new(p){
            match entry {
                Ok(ref entry) => {
                    if !is_markdown(&entry) {
                        continue;
                    }
                    if let Err(e) = parse_file(entry.path(), fr) {
                        handle_error(e)
                    }
                }
                Err(e) => handle_error(Error::from(e)),
            }
        }
    }

    Ok(())
}

fn is_markdown(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            if s.starts_with(".") {
                return false;
            }
            if s.ends_with(".md") || s.ends_with(".markdown") {
                return true;
            }
            false
        })
        .unwrap_or(false)
}

fn parse_file(path: &Path, fr: Format) -> Result<(), Error> {
    println!("{:?}", path);
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut buf_reader = BufReader::new(file);

    let mut delim = "";
    let mut front_matter = String::new();
    let mut contents = String::new();

    let mut start = false;
    let mut end = false;

    while let Some(line) = buf_reader.by_ref().lines().next() {
        if let Ok(line) = line {
            if !start {
                let trim = line.trim_start();
                if trim == "" {
                    continue;
                }
                let trim_str = trim.to_string();
                if trim_str.starts_with(TOML_DELIM) {
                    if fr.delimiter() == TOML_DELIM {
                        return Ok(());
                    }

                    delim = TOML_DELIM;
                    start = true;
                    continue;
                }
                if trim_str.starts_with(YAML_DELIM) {
                    if fr.delimiter() == YAML_DELIM {
                        return Ok(());
                    }
                    delim = YAML_DELIM;
                    start = true;
                    front_matter.push_str(&line);
                    front_matter.push('\n');
                    continue;
                }

                return Err(Error::InferFormat);
            }
            if line.trim_start().to_string().starts_with(delim) {
                end = true;
                break;
            }
            front_matter.push_str(&line);
            front_matter.push('\n');
        }
    }

    if !end {
        return Err(Error::InferFormat);
    }

    let infmt = Format::from_delim(delim).unwrap();
    let fm = FrontMatter::new(infmt, front_matter);

    if fr == Format::Toml {
        contents.push_str(TOML_DELIM);
        contents.push('\n');
    }
    let fmtext = fm.convert_to(fr)?.text;
    contents.push_str(&fmtext);
    contents.push_str(fr.delimiter());
    contents.push('\n');

    buf_reader.read_to_string(&mut contents)?;
    drop(buf_reader);

    let mut file = OpenOptions::new().truncate(true).write(true).open(path)?;
    file.write_all(contents.as_bytes())?;
    println!("changed");
    Ok(())
}

fn handle_error(error: Error) {
    println!("{}", error);
}
