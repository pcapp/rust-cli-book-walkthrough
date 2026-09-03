use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .author("peter.cappetto@gmail.com")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..)
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print a newline")
        )
        .get_matches();

    //let text: Vec<&String> = matches.get_many("text").unwrap().collect();
    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();

    let ending = if matches.get_flag("omit_newline") {""} else {"\n"};

    print!("{}{ending}", text.join(" "));
}
