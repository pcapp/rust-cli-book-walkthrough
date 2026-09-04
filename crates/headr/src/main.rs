use clap::{Arg, Command};

struct Args {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>
}

fn get_args() -> Args {
    let matches = Command::new("header")
        .author("peter.cappetto@gmail.com")
        .about("display the first lines of a file")
        .arg(
            Arg::new("file")
                .value_name("file")
                .default_value("-")
        )
        .arg(
            Arg::new("bytes")
                .value_name("bytes")
                .short('c')
                .long("bytes")
        )
        .arg(
            Arg::new("count")
                .value_name("count")
                .short('n')
                .long("lines")
         ).get_matches();



    Args {
        files: vec![],
        lines: 0,
        bytes: None
    }
}
fn main() {
    let _args = get_args();
}
