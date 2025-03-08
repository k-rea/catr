use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::distr::Alphanumeric;
use rand::Rng;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;
const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

fn gen_bad_file() -> String {
    let rng = rand::rng();
    loop {
        let filename: String = rng
            .clone()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().unwrap();

    assert!(output.status.success());

    let stdout = std::str::from_utf8(&output.stdout)
        .expect("invalid UTF-8")
        .to_string();
    assert_eq!(expected, stdout);
    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = std::str::from_utf8(&output.stdout)
        .expect("invalid UTF-8")
        .to_string();
    assert_eq!(expected, stdout);

    Ok(())
}

#[test]
fn stdin() -> TestResult {
    run_stdin(BUSTLE, &[], "tests/expected/the-bustle.txt.stdin.out")?;
    run_stdin(BUSTLE, &["-n"], "tests/expected/the-bustle.txt.n.stdin.out")?;
    run_stdin(BUSTLE, &["-b"], "tests/expected/the-bustle.txt.b.stdin.out")?;
    run_stdin(
        BUSTLE,
        &["-b", "-n"],
        "tests/expected/the-bustle.txt.nb.stdin.out",
    )
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")?;
    run(&[EMPTY, "-n"], "tests/expected/empty.txt.n.out")?;
    run(&[EMPTY, "-b"], "tests/expected/empty.txt.b.out")?;
    run(&[EMPTY, "-b", "-n"], "tests/expected/empty.txt.nb.out")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")?;
    run(&[FOX, "--number"], "tests/expected/fox.txt.n.out")?;
    run(&[FOX, "--number-nonblock"], "tests/expected/fox.txt.b.out")?;
    run(
        &[FOX, "--number-nonblock", "-n"],
        "tests/expected/fox.txt.nb.out",
    )
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")?;
    run(&[SPIDERS, "-n"], "tests/expected/spiders.txt.n.out")?;
    run(&[SPIDERS, "--number"], "tests/expected/spiders.txt.n.out")?;
    run(
        &[SPIDERS, "--number-nonblock"],
        "tests/expected/spiders.txt.b.out",
    )?;
    run(
        &[SPIDERS, "--number-nonblock", "-n"],
        "tests/expected/spiders.txt.nb.out",
    )
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")?;
    run(&[BUSTLE, "-n"], "tests/expected/the-bustle.txt.n.out")?;
    run(&[BUSTLE, "-b"], "tests/expected/the-bustle.txt.b.out")?;
    run(
        &[BUSTLE, "-b", "-n"],
        "tests/expected/the-bustle.txt.nb.out",
    )
}

#[test]
fn all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")?;
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")?;
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")?;
    run(
        &[FOX, SPIDERS, BUSTLE, "-b", "-n"],
        "tests/expected/all.nb.out",
    )
}
