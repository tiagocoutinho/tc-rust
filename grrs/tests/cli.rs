use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use std::io::Write;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("test/inexisting/file");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn find_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::NamedTempFile::new()?;
    writeln!(file, "Lets test\na brand new\nfile\nWith brand\nnew content")?;
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("brand").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a brand new\nWith brand\n"));
    Ok(())
}

#[test]
fn help_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("grrs <pattern> <path>"));
    Ok(())
}
