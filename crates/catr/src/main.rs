use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool
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
                .action(ArgAction::SetTrue)
                .help("Number lines")
        )
        .arg(
        Arg::new("number_nonblank")
            .short('b')
            .action(ArgAction::SetTrue)
            .help("Number non-blank lines")
    )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank: false
    }
}

fn main() {
    let args = get_args();
    println!("{:#?}", args);
}
