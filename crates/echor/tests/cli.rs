use std::fs;
use assert_cmd::Command;
use anyhow::Result;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicates::str::contains("Usage"));
    Ok(())
}

/*
    This is purposely non-canonical; however, I want to play around
    with stock Rust classes to see what we are gaining
    by using external crates.
 */
#[test]
fn stock_dies_no_args() -> Result<()> {
    use std::process::Command;

    let output = Command::new("../../target/debug/echor").output()?;
    // Status code 2 is an argument error
    assert_eq!(output.status.code().unwrap(), 2);
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["-n", "Hello there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    let output = cmd.args(args).output()?;

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}