use jsonwith::jsonformat;
use std::fs;

fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}

#[test]
fn jsonformat_sample1() {
    let raw = read("./tests/assets/sample1.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_sample2() {
    let raw = read("./tests/assets/sample2.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_sample3() {
    let raw = read("./tests/assets/sample3.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_emptydict() {
    let raw = read("./tests/assets/emptydict.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_emptylist() {
    let raw = read("./tests/assets/emptylist.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_numberlist() {
    let raw = read("./tests/assets/numberlist.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_nestedlist() {
    let raw = read("./tests/assets/nestedlist.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_rootstring() {
    let raw = read("./tests/assets/rootstring.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_rootnumber() {
    let raw = read("./tests/assets/rootnumber.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}

#[test]
fn jsonformat_rootlist() {
    let raw = read("./tests/assets/rootlist.json");
    assert_eq!(jsonformat(&raw, 2), raw);
}
