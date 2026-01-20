use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn stats_runs_on_sample_file() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("textkit"));

    cmd.args(["stats", "tests/fixtures/sample.txt"]);
    cmd.assert()
        .success()
        .stdout(contains("lines: 4\nwords: 8\nchars: 46\nbytes: 46"));
}

#[test]
fn uniq_on_sample_file() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("textkit"));

    cmd.args(["uniq", "--all", "tests/fixtures/sample.txt"]);
    cmd.assert()
        .success()
        .stdout(contains("First line\nSecond line\nThird line"));
}

#[test]
fn grep_on_sample_file() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("textkit"));

    cmd.args(["grep", "First", "-i", "-n", "tests/fixtures/sample.txt"]);
    cmd.assert().success().stdout(contains("1:First line"));
}
