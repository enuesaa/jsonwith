use std::process::Command;
use std::process::Stdio;

#[test]
fn json2yaml_stdin_given() {
    let output = Command::new("sh")
        .args(&["-c", "echo '{\"a\":\"b\"}' | cargo run json2yaml"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "a: b\n\n");
}
