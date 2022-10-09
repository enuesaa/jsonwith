use jsonwith_formatter::json2yaml;

#[test]
fn json2yaml_sample1() {
    let json_string = r#"
{
  "a": "a-value",
  "b": 100,
  "c": {
    "ckey1": "ckey1-value",
    "ckey2": "ckey2-value"
  },
  "d": true,
  "e": [
    {
      "e1key1": "e1key1-value",
      "e1key2": "e1key2-value"
    },
    {
      "e2key1": "e2key1-value"
    }
  ]
}
"#;

    let expect = String::from(
        r#"a: a-value
b: 100
c: 
  ckey1: ckey1-value
  ckey2: ckey2-value
d: true
e: 
- e1key1: e1key1-value
  e1key2: e1key2-value
- e2key1: e2key1-value
"#,
    );
    assert_eq!(json2yaml(json_string), expect);
}

#[test]
fn json2yaml_sample2() {
    let json_string = r#"
[
  "a",
  "b",
  {"c": "cv", "d": "dv", "e": {"e2": "e2v"}},
  {"d": "f"}
]
"#;

    let expect = String::from(
        r#"- a
- b
- c: cv
  d: dv
  e: 
    e2: e2v
- d: f
"#,
    );

    assert_eq!(json2yaml(json_string), expect);
}

#[test]
fn json2yaml_empty_dict() {
    let json_string = r#"
{"a": {}}
"#;

    let expect = String::from(
        r#"a: {}
"#,
    );

    assert_eq!(json2yaml(json_string), expect);
}

#[test]
fn json2yaml_empty_list() {
    let json_string = r#"
{"a": []}
"#;

    let expect = String::from(
        r#"a: []
"#,
    );

    assert_eq!(json2yaml(json_string), expect);
}

#[test]
fn json2yaml_number_list() {
  let json_string = r#"
{"a": [0, 1, 2]}
"#;

  let expect = String::from(
      r#"a: 
- 0
- 1
- 2
"#,
  );

  assert_eq!(json2yaml(json_string), expect);
}
