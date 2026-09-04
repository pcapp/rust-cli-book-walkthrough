use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::{Arg, Command, Parser};
use anyhow::Result;

#[derive(Parser, Debug)]
struct Args {
    file: Option<String>
}

fn open(filename: &Option<String>) -> Result<Box<dyn BufRead>> {
    match filename {
        None => {
            println!("Using stdin");
            Ok(Box::new(BufReader::new(io::stdin())))
        },
        Some(filename) => {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            Ok(Box::new(reader))
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut reader = match open(&args.file) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Unable to open {}: {err}", args.file.expect("filename"));
            std::process::exit(1);
        }
    };
    let mut contents = String::new();

    reader.read_to_string(&mut contents).expect("should read the file");

    println!("{contents}")
}
