use jsonwith::json2yaml;
use std::fs;

fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}

#[test]
fn json2yaml_sample1() {
    let json = read("./tests/assets/sample1.json");
    let expected = read("./tests/assets/sample1.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_sample2() {
    let json = read("./tests/assets/sample2.json");
    let expected = read("./tests/assets/sample2.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_sample3() {
    let json = read("./tests/assets/sample3.json");
    let expected = read("./tests/assets/sample3.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_emptydict() {
    let json = read("./tests/assets/emptydict.json");
    let expected = read("./tests/assets/emptydict.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_emptylist() {
    let json = read("./tests/assets/emptylist.json");
    let expected = read("./tests/assets/emptylist.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_numberlist() {
    let json = read("./tests/assets/numberlist.json");
    let expected = read("./tests/assets/numberlist.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_nestedlist() {
    let json = read("./tests/assets/nestedlist.json");
    let expected = read("./tests/assets/nestedlist.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_rootstring() {
    let json = read("./tests/assets/rootstring.json");
    let expected = read("./tests/assets/rootstring.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}

#[test]
fn json2yaml_rootlist() {
    let json = read("./tests/assets/rootlist.json");
    let expected = read("./tests/assets/rootlist.yaml");
    assert_eq!(json2yaml(&json, 2), expected);
}
