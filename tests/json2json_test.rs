use jsonwith_formatter::json2json;

#[test]
fn json2json_sample1() {
    let json_string = r#"{
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
    assert_eq!(json2json(json_string), String::from(json_string));
}

#[test]
fn json2json_sample2() {
    let json_string = r#"[
  "a",
  "b",
  {
    "c": "cv",
    "d": "dv",
    "e": {
      "e2": "e2v"
    }
  },
  {
    "d": "f"
  }
]
"#;

    assert_eq!(json2json(json_string), String::from(json_string));
}

#[test]
fn json2json_empty_dict() {
    let json_string = r#"
{"a": {}}
"#;
    let expect = String::from(
        r#"{
  "a": {}
}
"#,
    );

    assert_eq!(json2json(json_string), expect);
}

#[test]
fn json2json_empty_list() {
    let json_string = r#"
{"a": []}
"#;

    let expect = String::from(
        r#"{
  "a": []
}
"#,
    );

    assert_eq!(json2json(json_string), expect);
}
