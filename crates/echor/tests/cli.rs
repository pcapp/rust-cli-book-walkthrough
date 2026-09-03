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
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> Result<()> {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"]).assert().success().stdout(expected);
    Ok(())
}