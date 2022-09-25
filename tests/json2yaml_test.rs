use jsonwith_formatter::json2yaml;

#[test]
fn convert_json_to_yaml() {
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

  let out = json2yaml(json_string);
  assert_eq!(out, String::from(r#"a: a-value
b: 100
c: 
  ckey1: ckey1-value
  ckey2: ckey2-value
d: true
e: 
  - e1key1: e1key1-value
    e1key2: e1key2-value
  - e2key1: e2key1-value
"#));
}
