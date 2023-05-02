use jsonwith::json2yaml;
use jsonwith::util::read;

#[test]
fn json2yaml_sample1() {
    let sample1json = read("./tests/assets/sample1.json");
    let sample1yaml = read("./tests/assets/sample1.yaml");
    // todo fix last line existence. 
    assert_eq!(json2yaml(&sample1json, 2), sample1yaml);
}

#[test]
fn json2yaml_sample2() {
    let sample2json = read("./tests/assets/sample2.json");
    let sample2yaml = read("./tests/assets/sample2.yaml");
    assert_eq!(json2yaml(&sample2json, 2), sample2yaml);
}

#[test]
fn json2yaml_empty_dict() {
    let emptydictjson = read("./tests/assets/emptydict.json");
    let emptydictyaml = read("./tests/assets/emptydict.yaml");
    assert_eq!(json2yaml(&emptydictjson, 2), emptydictyaml);
}

#[test]
fn json2yaml_empty_list() {
    let emptylistjson = read("./tests/assets/emptylist.json");
    let emptylistyaml = read("./tests/assets/emptylist.yaml");
    assert_eq!(json2yaml(&emptylistjson, 2), emptylistyaml);
}

#[test]
fn json2yaml_number_list() {
    let numberlistjson = read("./tests/assets/numberlist.json");
    let numberlistyaml = read("./tests/assets/numberlist.yaml");
    assert_eq!(json2yaml(&numberlistjson, 2), numberlistyaml);
}
