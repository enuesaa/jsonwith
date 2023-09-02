use jsonwith::data::kv::Kv;
use jsonwith::data::kvs::Kvs;
use jsonwith::data::path::Path;
use jsonwith::data::tokens::Tokens;
use jsonwith::json::parse::parser::Parser;

#[test]
fn test_root_dict() {
    let text = "{\"a\": \"aaa\"}";
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
fn test_root_array() {
    let text = "[\"aaa\"]";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkArray),
            Kv::with(Path::from("$[0]"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$"), Tokens::EndArray),
        ]),
    );
}

#[test]
fn test_root_string() {
    let text = "\"aaa\"";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::String("aaa".to_string())),
        ]),
    );
}

#[test]
fn test_root_number() {
    let text = "107";
    let mut parser = Parser::new();
    let actual = parser.parse(text);
    println!("{:?}", actual);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::Number(107)),
        ]),
    );
}

#[test]
fn test_nested_dict() {
    let text = "{\"a\": \"aaa\", \"b\": {\"c\": \"ddd\"}, \"e\": 108}";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$.b"), Tokens::MkDict),
            Kv::with(Path::from("$.b.c"), Tokens::String("ddd".to_string())),
            Kv::with(Path::from("$.b"), Tokens::EndDict),
            Kv::with(Path::from("$.e"), Tokens::Number(108)),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}
