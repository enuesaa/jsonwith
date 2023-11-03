use jsonwith::data::kv::Kv;
use jsonwith::data::kvs::Kvs;
use jsonwith::data::path::Path;
use jsonwith::data::tokens::Tokens;
use jsonwith::yaml::parse::parser::Parser;

#[test]
fn test_normal() {
    let text = "a: aaa";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_number() {
    let text = "a: 99";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::Number(99)),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_bool() {
    let text = "a: true";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::Bool(true)),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_null() {
    let text = "a: null";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::Null),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_array() {
    let text = "a:\\n- b\\n- c";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::MkArray),
            Kv::with(Path::from("$.a[0]"), Tokens::String("b".to_string())),
            Kv::with(Path::from("$.a[1]"), Tokens::String("c".to_string())),
            Kv::with(Path::from("$.a"), Tokens::EndArray),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_nested_dict() {
    let text = "a:\\n  b: b-value\\n  c:\\n    d: d-value";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::MkDict),
            Kv::with(Path::from("$.a.b"), Tokens::String("b-value".to_string())),
            Kv::with(Path::from("$.a.c"), Tokens::MkDict),
            Kv::with(Path::from("$.a.c.d"), Tokens::String("d-value".to_string())),
            Kv::with(Path::from("$.a.c"), Tokens::EndDict),
            Kv::with(Path::from("$.a"), Tokens::EndDict),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_dict_in_array() {
    let text = "items:\\n- a:aa\\n  b:bb\\n- c:cc\\n- d:dd";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.items"), Tokens::MkArray),
            Kv::with(Path::from("$.items[0]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[0].a"), Tokens::String("aa".to_string())),
            Kv::with(Path::from("$.items[0].b"), Tokens::String("bb".to_string())),
            Kv::with(Path::from("$.items[0]"), Tokens::EndDict),
            Kv::with(Path::from("$.items[1]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[1].c"), Tokens::String("cc".to_string())),
            Kv::with(Path::from("$.items[1]"), Tokens::EndDict),
            Kv::with(Path::from("$.items[2]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[2].d"), Tokens::String("dd".to_string())),
            Kv::with(Path::from("$.items[2]"), Tokens::EndDict),
            Kv::with(Path::from("$.items"), Tokens::EndArray),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_append_close_tags_per_indent_size() {
    let text = "items:\\n- id: 1\\n  bb:\\n  - cc\\n  - dd\\n- id: 2\\n  bb:\\n  - ee";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.items"), Tokens::MkArray),
            Kv::with(Path::from("$.items[0]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[0].id"), Tokens::Number(1)),
            Kv::with(Path::from("$.items[0].bb"), Tokens::MkArray),
            Kv::with(Path::from("$.items[0].bb[0]"), Tokens::String("cc".to_string())),
            Kv::with(Path::from("$.items[0].bb[1]"), Tokens::String("dd".to_string())),
            Kv::with(Path::from("$.items[0].bb"), Tokens::EndArray),
            Kv::with(Path::from("$.items[0]"), Tokens::EndDict),
            Kv::with(Path::from("$.items[1]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[1].id"), Tokens::Number(2)),
            Kv::with(Path::from("$.items[1].bb"), Tokens::MkArray),
            Kv::with(Path::from("$.items[1].bb[0]"), Tokens::String("ee".to_string())),
            Kv::with(Path::from("$.items[1].bb"), Tokens::EndArray),
            Kv::with(Path::from("$.items[1]"), Tokens::EndDict),
            Kv::with(Path::from("$.items"), Tokens::EndArray),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_append_close_tags_per_indent_size_with_nested_dict() {
    let text = "items:\\n- id: 1\\n  bb:\\n  - cc\\n  - dd:ff\\n- id: 2\\n  bb:\\n  - ee";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.items"), Tokens::MkArray),
            Kv::with(Path::from("$.items[0]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[0].id"), Tokens::Number(1)),
            Kv::with(Path::from("$.items[0].bb"), Tokens::MkArray),
            Kv::with(Path::from("$.items[0].bb[0]"), Tokens::String("cc".to_string())),
            Kv::with(Path::from("$.items[0].bb[1]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[0].bb[1].dd"),  Tokens::String("ff".to_string())),
            Kv::with(Path::from("$.items[0].bb[1]"), Tokens::EndDict),
            Kv::with(Path::from("$.items[0].bb"), Tokens::EndArray),
            Kv::with(Path::from("$.items[0]"), Tokens::EndDict),
            Kv::with(Path::from("$.items[1]"), Tokens::MkDict),
            Kv::with(Path::from("$.items[1].id"), Tokens::Number(2)),
            Kv::with(Path::from("$.items[1].bb"), Tokens::MkArray),
            Kv::with(Path::from("$.items[1].bb[0]"), Tokens::String("ee".to_string())),
            Kv::with(Path::from("$.items[1].bb"), Tokens::EndArray),
            Kv::with(Path::from("$.items[1]"), Tokens::EndDict),
            Kv::with(Path::from("$.items"), Tokens::EndArray),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}
