use jsonwith::json2yamlv2;
use std::fs;

fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}

#[test]
fn json2yamlv2_sample1() {
    let sample1json = read("./tests/assets/sample1.json");
    let sample1yaml = read("./tests/assets/sample1.yaml");
    // todo fix last line existence. 
    assert_eq!(json2yamlv2(&sample1json, 2), sample1yaml);
}
