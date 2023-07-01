use jsonwith::json2yaml;
use std::fs;

fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}

#[test]
fn json2jsonv2_sample1() {
    let sample1 = read("./tests/assets/sample1.json");
    let sample1yaml = read("./tests/assets/sample1.yaml");
    assert_eq!(json2yaml(&sample1), sample1yaml);
}

#[test]
fn json2jsonv2_sample2() {
    let sample2 = read("./tests/assets/sample2.json");
    let sample2yaml = read("./tests/assets/sample2.yaml");
    assert_eq!(json2yaml(&sample2), sample2yaml);
}
