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
    assert_eq!(jsonformat(&raw), raw);
}

#[test]
fn jsonformat_sample2() {
    let raw = read("./tests/assets/sample2.json");
    assert_eq!(jsonformat(&raw), raw);
}

#[test]
fn jsonformat_minimum() {
    let raw = read("./tests/assets/minimum.json");
    assert_eq!(jsonformat(&raw), raw);
}

#[test]
fn jsonformat_emptydict() {
    let raw = read("./tests/assets/emptydict.json");
    assert_eq!(jsonformat(&raw), raw);
}

#[test]
fn jsonformat_emptylist() {
    let raw = read("./tests/assets/emptylist.json");
    assert_eq!(jsonformat(&raw), raw);
}

#[test]
fn jsonformat_numberlist() {
    let raw = read("./tests/assets/numberlist.json");
    assert_eq!(jsonformat(&raw), raw);
}
