use std::process::Command;// Run programs
use assert_cmd::prelude::*; // Add methods on commands

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("{{program_name}}")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("help"));;
    Ok(())
}

#[test]
fn fail_on_non_existing_file()-> Result<(), Box<std::error::Error>> {
    Command::cargo_bin("{{program_name}}")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();
    Ok(())
}