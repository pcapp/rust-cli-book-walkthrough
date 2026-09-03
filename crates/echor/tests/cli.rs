use std::fs;
use assert_cmd::Command;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure().stderr(predicates::str::contains("Usage"));
}

/*
    This is purposely non-canonical; however, I want to play around
    with stock Rust classes to see what we are gaining
    by using external crates.
 */
#[test]
fn stock_dies_no_args() {
    use std::process::Command;

    let output = Command::new("../../target/debug/echor").output().unwrap();
    // Status code 2 is an argument error
    assert_eq!(output.status.code().unwrap(), 2);
}

#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}