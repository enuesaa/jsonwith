use std::process::Command;
use std::process::Stdio;

#[test]
fn jsonformat_stdin_given() {

    let output = Command::new("sh")
        .args(&["-c", "echo '{\"a\":\"b\"}' | cargo run format"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "{\n  \"a\": \"b\"\n}\n\n");
}
