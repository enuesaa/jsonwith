use std::fs;
use jsonwith::json2jsonv2;

fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}

#[test]
fn json2jsonv2_sample1() {
    let sample1 = read("./tests/assets/sample1.json");
    assert_eq!(json2jsonv2(&sample1), sample1);
}

#[test]
fn json2jsonv2_sample2() {
    let sample2 = read("./tests/assets/sample2.json");
    assert_eq!(json2jsonv2(&sample2), sample2);
}

#[test]
fn json2jsonv2_empty_dict() {
    let emptydict = read("./tests/assets/emptydict.json");
    assert_eq!(json2jsonv2(&emptydict), emptydict); // should {\n  \"a\": {}\n}\n
}

#[test]
fn json2jsonv2_empty_list() {
    let emptylist = read("./tests/assets/emptylist.json");
    assert_eq!(json2jsonv2(&emptylist), emptylist); // should {\n  \"a\": []\n}\n
}
