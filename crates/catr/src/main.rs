use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::{Arg, ArgAction, Command};
use anyhow::{Result};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .author("peter.cappetto@gmail.com")
        .version("0.1.0")
        .about("Rust version of `cat`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files(s)")
                .default_value("-")
                .num_args(1..)
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue)
                .help("Number lines")
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .help("Number non-blank lines")
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        let mut file = match open(&filename) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Unable to open {filename}: {err}");
                continue
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut num_nonblank = 0;

        for (index, line) in contents.lines().enumerate() {
            let prefix = if args.number_lines {
                format!("{:>6}\t", index + 1)
            }
            else if args.number_nonblank_lines {
                if line.is_empty() {
                    String::new()
                }
                else {
                    num_nonblank += 1;
                    format!("{:>6}\t", num_nonblank)
                }
            }
            else {
                String::new()
            };

            println!("{prefix}{line}");
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
